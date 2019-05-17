use super::*;
use crate::parse::parse_glyph;

#[test]
fn transform() {
    let transform = AffineTransform::default();
    assert_eq!(transform.x_scale, 1.0);
}

#[test]
fn parse() {
    let glyph = parse_glyph(TEST_GLYPH.as_bytes()).unwrap();
    assert_eq!(&glyph.name, "period");
}

#[test]
fn parse_utf16() {
    let bytes = include_bytes!("../../testdata/utf16-glyph.xml");
    let glyph = parse_glyph(bytes).unwrap();
    assert_eq!(glyph.width, Some(268.));
}

static TEST_GLYPH: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<glyph name="period" format="2">
  <advance width="268"/>
  <unicode hex="002E"/>
  <image fileName="period sketch.png" xScale="0.5" yScale="0.5"/>
  <guideline y="-12" name="overshoot"/>
  <anchor x="74" y="197" name="top"/>
  <outline>
    <contour>
      <point x="237" y="152"/>
      <point x="193" y="187"/>
      <point x="134" y="187" type="curve" smooth="yes"/>
      <point x="74" y="187"/>
      <point x="30" y="150"/>
      <point x="30" y="88" type="curve" smooth="yes"/>
      <point x="30" y="23"/>
      <point x="74" y="-10"/>
      <point x="134" y="-10" type="curve" smooth="yes"/>
      <point x="193" y="-10"/>
      <point x="237" y="25"/>
      <point x="237" y="88" type="curve" smooth="yes"/>
    </contour>
  </outline>
  <lib>
    <dict>
      <key>com.letterror.somestuff</key>
      <string>arbitrary custom data!</string>
      <key>public.markColor</key>
      <string>1,0,0,0.5</string>
      <key>public.postscript.hints</key>
      <dict>
        <key>formatVersion</key><string>1</string>
        <key>id</key><string>w268c237,88 237,152 193,187c134,187 74,187 30,150c30,88 30,23 74,-10c134,-10 193,-10 237,25</string>
        <key>hintSetList</key>
        <array>
          <dict>
            <key>pointTag</key>
            <string>hintSet0000</string>
            <key>stems</key>
            <array>
              <string>hstem -10 197</string>
              <string>vstem 30 207</string>
            </array>
          </dict>
          <dict>
            <key>pointTag</key>
            <string>hintSet0004</string>
            <key>stems</key>
            <array>
              <string>hstem 11 -21</string>
              <string>vstem 30 207</string>
            </array>
          </dict>
        </array>
      </dict>
    </dict>
  </lib>
</glyph>
"#;
