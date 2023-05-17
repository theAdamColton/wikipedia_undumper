use wikipedia_undumper::schema::Page;
use quick_xml::DeError;
use quick_xml::de::from_str;

const XML_PAGE_REDIRECT: &str = r#"  
<page>
    <title>American Units Of Measurement</title>
    <ns>0</ns>
    <id>41</id>
    <redirect title="United States customary units"/>
    <revision>
      <id>4691442</id>
      <parentid>36394</parentid>
      <timestamp>2014-01-15T07:11:55Z</timestamp>
      <contributor>
        <username>AvicBot</username>
        <id>114482</id>
      </contributor>
      <minor/>
      <comment>Robot: Fixing double redirect to [[United States customary units]]</comment>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes="43" xml:space="preserve">#REDIRECT [[United States customary units]]</text>
      <sha1>ge4o55xxsek0ma2fce8zr5429o9be0z</sha1>
    </revision>
  </page>"#;


#[cfg(test)]
mod page_tests {

    use super::*;

    #[test]
    fn test_deserialize_page() {
        let p: Result<Page, DeError> = from_str(XML_PAGE_REDIRECT);
        println!("{:?}", p);
        assert!(p.is_ok());
    }
}
