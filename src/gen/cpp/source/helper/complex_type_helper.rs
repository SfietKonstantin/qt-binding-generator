use def::Property;
use std::io::{Result, Write};

const QSTRING_HELPER: &str = r"    typedef void (*QStringSet)(QString *, const char *, int);
    void setQString(QString *returned, const char *input, int count) {
        *returned = QString::fromUtf8(input, count);
    }
";

pub(super) fn gen_qstring_helper<W>(write: &mut W, properties: &Vec<&Property>) -> Result<()>
where
    W: Write,
{
    let has_qstring = properties
        .iter()
        .find(|property| property.variable_type().cpp_type_kind().is_qstring())
        .is_some();

    if has_qstring {
        write!(write, "{}", QSTRING_HELPER)?;
    }

    Ok(())
}

const QBYTEARRAY_HELPER: &str =
    r"    typedef void (*QByteArraySet)(QByteArray *, const char *, int);
    void setQByteArray(QByteArray* returned, const char* input, int count) {
        *returned = QByteArray(bytes, count);
    }
";

pub(super) fn gen_qbytearray_helper<W>(write: &mut W, properties: &Vec<&Property>) -> Result<()>
where
    W: Write,
{
    let has_qstring = properties
        .iter()
        .find(|property| property.variable_type().cpp_type_kind().is_qbytearray())
        .is_some();

    if has_qstring {
        write!(write, "{}", QBYTEARRAY_HELPER)?;
    }

    Ok(())
}
