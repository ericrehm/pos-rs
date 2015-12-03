//! SBET file format.

use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::IntoIterator;
use std::path::Path;

use byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

use {Error, Result};
use point::Point;
use units::Radians;

/// An SBET reader.
#[derive(Debug)]
pub struct Reader<R: Read> {
    reader: R,
}

impl Reader<BufReader<File>> {
    /// Opens a reader for a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::sbet::Reader;
    /// let reader = Reader::from_path("data/2-points.sbet").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>> {
        Ok(Reader { reader: BufReader::new(try!(File::open(path))) })
    }
}

impl<R: Read> Reader<R> {
    /// Reads a point from this reader.
    ///
    /// Returns none if the file is at its end when this reader starts reading. We have to do it
    /// this way since sbet files don't have a point count.
    ///
    /// # Examples
    ///
    /// ```
    /// use pos::sbet::Reader;
    /// let mut reader = Reader::from_path("data/2-points.sbet").unwrap();
    /// let point = reader.read_point().unwrap().unwrap();
    /// ```
    pub fn read_point(&mut self) -> Result<Option<Point>> {
        let time = match self.reader.read_f64::<LittleEndian>() {
            Ok(time) => time,
            Err(byteorder::Error::UnexpectedEOF) => return Ok(None),
            Err(err) => return Err(Error::from(err)),
        };
        Ok(Some(Point {
            time: time,
            latitude: Radians(try!(self.reader.read_f64::<LittleEndian>())),
            longitude: Radians(try!(self.reader.read_f64::<LittleEndian>())),
            altitude: try!(self.reader.read_f64::<LittleEndian>()),
            x_velocity: Some(try!(self.reader.read_f64::<LittleEndian>())),
            y_velocity: Some(try!(self.reader.read_f64::<LittleEndian>())),
            z_velocity: Some(try!(self.reader.read_f64::<LittleEndian>())),
            roll: Radians(try!(self.reader.read_f64::<LittleEndian>())),
            pitch: Radians(try!(self.reader.read_f64::<LittleEndian>())),
            yaw: Radians(try!(self.reader.read_f64::<LittleEndian>())),
            wander_angle: Some(Radians(try!(self.reader.read_f64::<LittleEndian>()))),
            x_acceleration: Some(try!(self.reader.read_f64::<LittleEndian>())),
            y_acceleration: Some(try!(self.reader.read_f64::<LittleEndian>())),
            z_acceleration: Some(try!(self.reader.read_f64::<LittleEndian>())),
            x_angular_rate: Some(Radians(try!(self.reader.read_f64::<LittleEndian>()))),
            y_angular_rate: Some(Radians(try!(self.reader.read_f64::<LittleEndian>()))),
            z_angular_rate: Some(Radians(try!(self.reader.read_f64::<LittleEndian>()))),
            ..Default::default()
        }))
    }
}

impl<R: Read> IntoIterator for Reader<R> {
    type Item = Point;
    type IntoIter = ReaderIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        ReaderIterator { reader: self }
    }
}

/// An iterator over an sbet reader.
#[derive(Debug)]
pub struct ReaderIterator<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> Iterator for ReaderIterator<R> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read_point().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let reader = Reader::from_path("data/2-points.sbet").unwrap();
        let points: Vec<_> = reader.into_iter().collect();
        assert_eq!(2, points.len());
    }
}
