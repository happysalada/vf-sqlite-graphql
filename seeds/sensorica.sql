INSERT INTO actions (id, name, input_output) VALUES ('01FBXZBD20NTJSBSHFT3HMQN9M', 'work', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZEE073FSKRD1J49QGQ2G0', 'produce', 'Output');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZFT2C9Z4DTWRNNRMSTKT7', 'consume', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZHHA2YHTAP902Y6FKZ885', 'cite', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZHY6SD354TSP6G41W0XWY', 'accept', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZJ9JDCZ6YXJY6BVR7DAWT', 'modify', 'Output');
INSERT INTO actions (id, name, input_output) VALUES ('01FBZMRP9708ZKQ64RV3A09CSD', 'use', 'Input');
INSERT INTO units (id, label) VALUES ('01FBXZS196WG2YSN1YKYDSG0W8', 'hour');
INSERT INTO units (id, label) VALUES ('01FBXZSF3CVKKA8JBAWD7VDG7G', 'each');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMSZT0Y1A4VQS6839Y12GGYT', 'tibuerius_brastaviceanu', 'Tiberius Brastaviceanu', 'tiberius.brastaviceanu@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMSZT99637VPN57REMAZWTTE', 'alex_dicu', 'Alex Dicu', 'alex8dicu@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT8S0WZ5HHS1BFTYFPGBNFR', 'dounia_saeme', 'Dounia Saeme', 'dsaeme@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT90M1D63DWDCSJD4C36V3Z', 'sebastian_klemm', 'Sebastian Klemm', 'an.sebastian.klemm@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT99WEK05ETH0T0BJ5H05QN', 'alexis_alonso', 'Alexis Alonso', 'alonfastus@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT9J77C2TQ1JY6KZQXHEPSG', 'chad', 'Chad', 'greenlynxsolutions1969@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT9MAR2H8B30XQCWC4WMSBQ', 'charlize_de_beer', 'Charlize de Beer', 'charlizedebeer94@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT9PG71GJ62859AF3KR8E67', 'elie_el_haddad', 'Elie El Haddad', 'elie.g.hd@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT9XKJN6TH07KXB3PMCT9HW', 'mariok', 'Mariok', 'Mariokodsi94@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT9ZDCC962ESY1CZ0HZN1BV', 'mayssam', 'Mayssam', 'mayssamdaaboul@gmail.com', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMTA1PPAEMSC8MAT6R3JEM33', 'ross_tieman', 'Ross Tieman', 'ross@allfed.info', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMTA3CXTDG6QW9RK1WZPG38P', 'unai', 'Unai', 'unai@shipo-tz.org', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMSZZZBV02GSXTANEJCR4P04', 'portable_sauna', 'Portable Sauna', 'portable.sauna@sensorica.co', 'Project');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT07SPZZA3D1XXGPNPXV28M', 'sensorica', 'Sensorica', 'sensorica@sensorica.co', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT0GQ6TQ2EEWKH1A3Y33WNC', 'greens_for_good', 'Greens for good', 'greens.for.good@sensorica.co', 'Project');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT8VXWAWQ2PXWB2K908W2RR', 'rope_maker', 'Rope maker', 'rope.maker@sensorica.co', 'Project');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMT9SXZZKAT6SV39XEED9Z96', 'joshua_m_pearce_michigan_tech_university', ''Joshua M. Pearce - Michigan Tech University, '', 'Organization');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMT02BYT2TSXH3CPG5EWHSWW', 'Affiliate');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMT0AW37CPHYBJ184YWDTNRX', 'Representative');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMT02J76M7FB4RCX3QSRN1EX', 'Part of');
/* sensorica relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT096N710E7Z8Q695GGAFFK', '01FMSZT0Y1A4VQS6839Y12GGYT', '01FMT07SPZZA3D1XXGPNPXV28M', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT09CA0F5SB6Q8B2VYKGFJJ', '01FMSZT99637VPN57REMAZWTTE', '01FMT07SPZZA3D1XXGPNPXV28M', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT8YRAQCDV0Z8GS00YZ53YQ', '01FMT8S0WZ5HHS1BFTYFPGBNFR', '01FMT07SPZZA3D1XXGPNPXV28M', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT95ECZ1QG7WA9YBA9M9Y64', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT07SPZZA3D1XXGPNPXV28M', '01FMT02J76M7FB4RCX3QSRN1EX');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT95PV5PBTCYEW8W988MERH', '01FMT8VXWAWQ2PXWB2K908W2RR', '01FMT07SPZZA3D1XXGPNPXV28M', '01FMT02J76M7FB4RCX3QSRN1EX');
/* Portable sauna relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT03JJKQ21HJGZ65RC0N5D2', '01FMSZT0Y1A4VQS6839Y12GGYT', '01FMSZZZBV02GSXTANEJCR4P04', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT03THP3XMFPHPCAK8S2V21', '01FMSZT99637VPN57REMAZWTTE', '01FMSZZZBV02GSXTANEJCR4P04', '01FMT02BYT2TSXH3CPG5EWHSWW');
/* Rope maker relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT8S0WZ5HHS1BFTYFPGBNFR', '01FMT8S0WZ5HHS1BFTYFPGBNFR', '01FMT8VXWAWQ2PXWB2K908W2RR', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT0K9BG1A47TXEW55G78KFC', '01FMSZT0Y1A4VQS6839Y12GGYT', '01FMT8VXWAWQ2PXWB2K908W2RR', '01FMT0AW37CPHYBJ184YWDTNRX');
/* greens for good relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT90M1D63DWDCSJD4C36V3Z', '01FMT90M1D63DWDCSJD4C36V3Z', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9GVP4Y80J1AS5CJG0G3QY', '01FMT99WEK05ETH0T0BJ5H05QN', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9JPNDYV6ACBDBNQ9ACEX0', '01FMT9J77C2TQ1JY6KZQXHEPSG', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9N09PWCS9A134VC7SP5D1', '01FMT9MAR2H8B30XQCWC4WMSBQ', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9PW40Q8WVM0ZZ8Z9NXPY8', '01FMT9PG71GJ62859AF3KR8E67', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9TBMDEA0T5WXW81A6WE33', '01FMT9SXZZKAT6SV39XEED9Z96', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9Y14E52JP1N2RR7SRTFW9', '01FMT9XKJN6TH07KXB3PMCT9HW', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMT9ZRZ94RV2HA93P0CVE518', '01FMT9ZDCC962ESY1CZ0HZN1BV', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMTA236VSH556YA0PQ48T2ST', '01FMTA1PPAEMSC8MAT6R3JEM33', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMTA3RS0MRQZ3NETAVVKSM8E', '01FMTA3CXTDG6QW9RK1WZPG38P', '01FMT0GQ6TQ2EEWKH1A3Y33WNC', '01FMT02BYT2TSXH3CPG5EWHSWW');
