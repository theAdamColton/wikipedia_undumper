use quick_xml::de::from_str;
use quick_xml::DeError;
use wikipedia_undumper::schema::{Namespace, Page};

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

const XML_PAGE_NORMAL: &str = r#"
  <page>
    <title>Abrahamic religion</title>
    <ns>0</ns>
    <id>43</id>
    <revision>
      <id>7782264</id>
      <parentid>7782230</parentid>
      <timestamp>2021-09-20T20:48:56Z</timestamp>
      <contributor>
        <ip>2603:6081:2A07:61B7:A1DC:D620:40E5:DD2B</ip>
      </contributor>
      <comment>Reverted a mistake</comment>
      <model>wikitext</model>
      <format>text/x-wiki</format>
      <text bytes="1262" xml:space="preserve">[[File:Three Main Abrahamic Religions.svg|200px|thumb|[[Religious symbol|Symbols]] of the three largest Abrahamic religions: the>

An '''Abrahamic Religion''' is a [[religion]] whose followers believe in the prophet [[Abraham]]. They believe Abraham and his sons/grandsons hold an important role in human >

True Abrahamic religions are [[monotheism|monotheistic]] (the belief that there is only one God). They also all believe that people should [[pray]] to God and [[worship]] God>

[[Category:Abrahamic religions| ]]</text>
      <sha1>t74r2re61ekfoz4uxqnoryinpc23fg1</sha1>
    </revision>
  </page>
"#;

#[cfg(test)]
mod page_tests {

    use super::*;

    #[test]
    fn test_deserialize_pages_is_ok() {
        let p: Result<Page, DeError> = from_str(XML_PAGE_REDIRECT);
        println!("{:?}", p);
        assert!(p.is_ok());
        let p: Result<Page, DeError> = from_str(XML_PAGE_NORMAL);
        println!("{:?}", p);
        assert!(p.is_ok());
    }

    #[test]
    fn test_get_page_properties() {
        let p: Page = from_str(XML_PAGE_NORMAL).unwrap();
        assert!(p.id == 43);
        assert!(p.ns == Namespace::Main);
        assert!(p.revisions.len() == 1);
        assert!(p.revisions[0].id == 7782264);
        assert!(p.revisions[0].comment == Some("Reverted a mistake".to_string()));
        assert!(p.revisions[0].model == "wikitext".to_string());
        assert!(p.revisions[0].format == "text/x-wiki".to_string());
        assert!(p.revisions[0].parentid == 7782230);
        assert!(
            p.revisions[0].contributor.ip
                == Some("2603:6081:2A07:61B7:A1DC:D620:40E5:DD2B".to_string())
        );
    }
}
