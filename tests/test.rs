extern crate nom;
extern crate crushtool;
use crushtool::{CrushMap, BucketTypes, CrushBucketStraw, OpCode, BucketAlg, CrushRuleStep, Bucket,
                CrushRuleMask, Rule, RuleType, decode_crushmap, encode_crushmap};

#[test]
fn test_decode_crushmap() {
    let crushmap_compiled: &[u8] =
        &[0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00,
          0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x0a, 0x00, 0x04, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0xfd, 0xff,
          0xff, 0xff, 0xfc, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0x01, 0x00, 0x04, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xfd, 0xff, 0xff, 0xff,
          0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
          0xfc, 0xff, 0xff, 0xff, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
          0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0a,
          0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x06, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x01, 0x00, 0x00, 0x00, 0x04,
          0x00, 0x00, 0x00, 0x68, 0x6f, 0x73, 0x74, 0x02, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00,
          0x00, 0x63, 0x68, 0x61, 0x73, 0x73, 0x69, 0x73, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00,
          0x00, 0x00, 0x72, 0x61, 0x63, 0x6b, 0x04, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
          0x72, 0x6f, 0x77, 0x05, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x70, 0x64, 0x75,
          0x06, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x70, 0x6f, 0x64, 0x07, 0x00, 0x00,
          0x00, 0x04, 0x00, 0x00, 0x00, 0x72, 0x6f, 0x6f, 0x6d, 0x08, 0x00, 0x00, 0x00, 0x0a,
          0x00, 0x00, 0x00, 0x64, 0x61, 0x74, 0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x09,
          0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x0a,
          0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x72, 0x6f, 0x6f, 0x74, 0x07, 0x00, 0x00,
          0x00, 0xfc, 0xff, 0xff, 0xff, 0x0e, 0x00, 0x00, 0x00, 0x69, 0x70, 0x2d, 0x31, 0x37,
          0x32, 0x2d, 0x33, 0x31, 0x2d, 0x34, 0x2d, 0x35, 0x36, 0xfd, 0xff, 0xff, 0xff, 0x0e,
          0x00, 0x00, 0x00, 0x69, 0x70, 0x2d, 0x31, 0x37, 0x32, 0x2d, 0x33, 0x31, 0x2d, 0x32,
          0x32, 0x2d, 0x32, 0xfe, 0xff, 0xff, 0xff, 0x10, 0x00, 0x00, 0x00, 0x69, 0x70, 0x2d,
          0x31, 0x37, 0x32, 0x2d, 0x33, 0x31, 0x2d, 0x34, 0x33, 0x2d, 0x31, 0x34, 0x37, 0xff,
          0xff, 0xff, 0xff, 0x07, 0x00, 0x00, 0x00, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
          0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x2e, 0x30, 0x01,
          0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x2e, 0x31, 0x02, 0x00,
          0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x2e, 0x32, 0x01, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x72, 0x65, 0x70, 0x6c, 0x69,
          0x63, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x65, 0x74, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
          0x00, 0x00, 0x01];

    println!("crushmap compiled len {}", crushmap_compiled.len());
    let expected_result = CrushMap {
        magic: 65536,
        max_buckets: 8,
        max_rules: 1,
        max_devices: 3,
        buckets: vec![BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              struct_size: 4,
                              id: -1,
                              bucket_type: OpCode::SetChooseLocalTries,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 3,
                              items: vec![(-2, Some("ip-172-31-43-147".to_string())),
                                          (-3, Some("ip-172-31-22-2".to_string())),
                                          (-4, Some("ip-172-31-4-56".to_string()))],
                              perm_n: 0,
                              perm: 3,
                          },
                          item_weights: vec![(0, 0), (0, 0), (0, 0)],
                      }),
                      BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              id: -2,
                              struct_size: 4,
                              bucket_type: OpCode::Take,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 1,
                              items: vec![(0, Some("osd.0".to_string()))],
                              perm_n: 0,
                              perm: 1,
                          },
                          item_weights: vec![(0, 0)],
                      }),
                      BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              id: -3,
                              struct_size: 4,
                              bucket_type: OpCode::Take,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 1,
                              items: vec![(1, Some("osd.1".to_string()))],
                              perm_n: 0,
                              perm: 1,
                          },
                          item_weights: vec![(0, 0)],
                      }),
                      BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              id: -4,
                              struct_size: 4,
                              bucket_type: OpCode::Take,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 1,
                              items: vec![(2, Some("osd.2".to_string()))],
                              perm_n: 0,
                              perm: 1,
                          },
                          item_weights: vec![(0, 0)],
                      }),
                      BucketTypes::Unknown,
                      BucketTypes::Unknown,
                      BucketTypes::Unknown,
                      BucketTypes::Unknown],
        rules: vec![Some(Rule {
                        len: 3,
                        mask: CrushRuleMask {
                            ruleset: 0,
                            rule_type: RuleType::Replicated,
                            min_size: 1,
                            max_size: 10,
                        },
                        steps: vec![CrushRuleStep {
                                        op: OpCode::Take,
                                        arg1: (-1, None),
                                        arg2: (0, Some("osd".to_string())),
                                    },
                                    CrushRuleStep {
                                        op: OpCode::ChooseLeafFirstN,
                                        arg1: (0, Some("osd".to_string())),
                                        arg2: (1, Some("host".to_string())),
                                    },
                                    CrushRuleStep {
                                        op: OpCode::Emit,
                                        arg1: (0, Some("osd".to_string())),
                                        arg2: (0, Some("osd".to_string())),
                                    }],
                    })],
        type_map: vec![(0, "osd".to_string()),
                       (1, "host".to_string()),
                       (2, "chassis".to_string()),
                       (3, "rack".to_string()),
                       (4, "row".to_string()),
                       (5, "pdu".to_string()),
                       (6, "pod".to_string()),
                       (7, "room".to_string()),
                       (8, "datacenter".to_string()),
                       (9, "region".to_string()),
                       (10, "root".to_string())],
        name_map: vec![(-4, "ip-172-31-4-56".to_string()),
                       (-3, "ip-172-31-22-2".to_string()),
                       (-2, "ip-172-31-43-147".to_string()),
                       (-1, "default".to_string()),
                       (0, "osd.0".to_string()),
                       (1, "osd.1".to_string()),
                       (2, "osd.2".to_string())],
        rule_name_map: vec![(0, "replicated_ruleset".to_string())],
        choose_local_tries: Some(0),
        choose_local_fallback_tries: Some(0),
        choose_total_tries: Some(50),
        chooseleaf_descend_once: Some(1),
        chooseleaf_vary_r: Some(0),
        straw_calc_version: Some(1),
        choose_tries: None,
    };
    let result = decode_crushmap(&crushmap_compiled);
    println!("crushmap {:?}", result);
    assert_eq!(Ok(expected_result), result);
}

#[test]
fn test_encode_crushmap() {
    let expected_result: Vec<u8> = vec![
        0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03,
        0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x0a, 0x00, 0x04, 0x00, 0x00,
        0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0xfd, 0xff, 0xff, 0xff, 0xfc,
        0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xfe,
        0xff, 0xff, 0xff,
        0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xfd,
        0xff, 0xff, 0xff,
        0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xfc,
        0xff, 0xff, 0xff,
        0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03,
        0x00, 0x00, 0x00,
        0x00, 0x01, 0x01, 0x0a, 0x01, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00,
        0x00, 0x00, 0x00,
        0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04,
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x68,
        0x6f, 0x73, 0x74, 0x02, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x63, 0x68,
        0x61, 0x73, 0x73,
        0x69, 0x73, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x72, 0x61, 0x63,
        0x6b, 0x04, 0x00,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x72, 0x6f, 0x77, 0x05, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00,
        0x00, 0x70, 0x64, 0x75, 0x06, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x70,
        0x6f, 0x64, 0x07,
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x72, 0x6f, 0x6f, 0x6d, 0x08, 0x00,
        0x00, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x64, 0x61, 0x74, 0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72,
        0x09, 0x00, 0x00,
        0x00, 0x06, 0x00, 0x00, 0x00, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x0a, 0x00,
        0x00, 0x00, 0x04,
        0x00, 0x00, 0x00, 0x72, 0x6f, 0x6f, 0x74, 0x07, 0x00, 0x00, 0x00, 0xfc, 0xff,
        0xff, 0xff, 0x0e,
        0x00, 0x00, 0x00, 0x69, 0x70, 0x2d, 0x31, 0x37, 0x32, 0x2d, 0x33, 0x31, 0x2d,
        0x34, 0x2d, 0x35,
        0x36, 0xfd, 0xff, 0xff, 0xff, 0x0e, 0x00, 0x00, 0x00, 0x69, 0x70, 0x2d, 0x31,
        0x37, 0x32, 0x2d,
        0x33, 0x31, 0x2d, 0x32, 0x32, 0x2d, 0x32, 0xfe, 0xff, 0xff, 0xff, 0x10, 0x00,
        0x00, 0x00, 0x69,
        0x70, 0x2d, 0x31, 0x37, 0x32, 0x2d, 0x33, 0x31, 0x2d, 0x34, 0x33, 0x2d, 0x31,
        0x34, 0x37, 0xff,
        0xff, 0xff, 0xff, 0x07, 0x00, 0x00, 0x00, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
        0x74, 0x00, 0x00,
        0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x2e, 0x30, 0x01, 0x00,
        0x00, 0x00, 0x05,
        0x00, 0x00, 0x00, 0x6f, 0x73, 0x64, 0x2e, 0x31, 0x02, 0x00, 0x00, 0x00, 0x05,
        0x00, 0x00, 0x00,
        0x6f, 0x73, 0x64, 0x2e, 0x32, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x12, 0x00, 0x00,
        0x00, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x72,
        0x75, 0x6c, 0x65,
        0x73, 0x65, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00,
        0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    let crushmap = CrushMap {
        magic: 65536,
        max_buckets: 8,
        max_rules: 1,
        max_devices: 3,
        buckets: vec![BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              struct_size: 4,
                              id: -1,
                              bucket_type: OpCode::SetChooseLocalTries,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 3,
                              items: vec![(-2, None), (-3, None), (-4, None)],
                              perm_n: 0,
                              perm: 3,
                          },
                          item_weights: vec![(0, 0), (0, 0), (0, 0)],
                      }),
                      BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              id: -2,
                              struct_size: 4,
                              bucket_type: OpCode::Take,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 1,
                              items: vec![(0, None)],
                              perm_n: 0,
                              perm: 1,
                          },
                          item_weights: vec![(0, 0)],
                      }),
                      BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              id: -3,
                              struct_size: 4,
                              bucket_type: OpCode::Take,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 1,
                              items: vec![(1, None)],
                              perm_n: 0,
                              perm: 1,
                          },
                          item_weights: vec![(0, 0)],
                      }),
                      BucketTypes::Straw(CrushBucketStraw {
                          bucket: Bucket {
                              id: -4,
                              struct_size: 4,
                              bucket_type: OpCode::Take,
                              alg: BucketAlg::Straw,
                              hash: 0,
                              weight: 0,
                              size: 1,
                              items: vec![(2, None)],
                              perm_n: 0,
                              perm: 1,
                          },
                          item_weights: vec![(0, 0)],
                      }),
                      BucketTypes::Unknown,
                      BucketTypes::Unknown,
                      BucketTypes::Unknown,
                      BucketTypes::Unknown],
        rules: vec![Some(Rule {
                        len: 3,
                        mask: CrushRuleMask {
                            ruleset: 0,
                            rule_type: RuleType::Replicated,
                            min_size: 1,
                            max_size: 10,
                        },
                        steps: vec![CrushRuleStep {
                                        op: OpCode::Take,
                                        arg1: (-1, None),
                                        arg2: (0, None),
                                    },
                                    CrushRuleStep {
                                        op: OpCode::ChooseLeafFirstN,
                                        arg1: (0, None),
                                        arg2: (1, None),
                                    },
                                    CrushRuleStep {
                                        op: OpCode::Emit,
                                        arg1: (0, None),
                                        arg2: (0, None),
                                    }],
                    })],
        type_map: vec![(0, "osd".to_string()),
                       (1, "host".to_string()),
                       (2, "chassis".to_string()),
                       (3, "rack".to_string()),
                       (4, "row".to_string()),
                       (5, "pdu".to_string()),
                       (6, "pod".to_string()),
                       (7, "room".to_string()),
                       (8, "datacenter".to_string()),
                       (9, "region".to_string()),
                       (10, "root".to_string())],
        name_map: vec![(-4, "ip-172-31-4-56".to_string()),
                       (-3, "ip-172-31-22-2".to_string()),
                       (-2, "ip-172-31-43-147".to_string()),
                       (-1, "default".to_string()),
                       (0, "osd.0".to_string()),
                       (1, "osd.1".to_string()),
                       (2, "osd.2".to_string())],
        rule_name_map: vec![(0, "replicated_ruleset".to_string())],
        choose_local_tries: Some(0),
        choose_local_fallback_tries: Some(0),
        choose_total_tries: Some(50),
        chooseleaf_descend_once: Some(1),
        chooseleaf_vary_r: Some(0),
        straw_calc_version: Some(1),
        choose_tries: None,
    };
    let result = encode_crushmap(crushmap);
    assert_eq!(expected_result, result.unwrap());
}