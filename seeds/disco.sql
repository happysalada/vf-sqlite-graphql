INSERT INTO actions (id, name, input_output) VALUES ('01FBXZBD20NTJSBSHFT3HMQN9M', 'work', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZEE073FSKRD1J49QGQ2G0', 'produce', 'Output');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZFT2C9Z4DTWRNNRMSTKT7', 'consume', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZHHA2YHTAP902Y6FKZ885', 'cite', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZHY6SD354TSP6G41W0XWY', 'accept', 'Input');
INSERT INTO actions (id, name, input_output) VALUES ('01FBXZJ9JDCZ6YXJY6BVR7DAWT', 'modify', 'Output');
INSERT INTO actions (id, name, input_output) VALUES ('01FBZMRP9708ZKQ64RV3A09CSD', 'use', 'Input');
INSERT INTO units (id, label) VALUES ('01FBXZS196WG2YSN1YKYDSG0W8', 'hour');
INSERT INTO units (id, label) VALUES ('01FBXZSF3CVKKA8JBAWD7VDG7G', 'each');
INSERT INTO labels (id, name, unique_name, color) VALUES ('01FMDPZGNMEQ2APXMDTXVDJ1S8', 'livelihood', 'livelihood', 'green');
INSERT INTO labels (id, name, unique_name, color) VALUES ('01FMDPZGNMX93K6WG0R4VQQ1BY', 'love', 'love', 'pink');
INSERT INTO labels (id, name, unique_name, color) VALUES ('01FMDPZGNMMTA1N6X3FMD8K729', 'care', 'care', 'red');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8A9QQBV77N1B1YP7R89RR0', 'stacco', 'Stacco', 'stacco@disco.coop', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AD71VJ3JG9WRP85XN053M', 'irene', 'Irene', 'irene@disco.coop', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AEBZT3AKMV0DH98ETPFTR', 'sari', 'Sari', 'sari@disco.coop', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AFR52CMRHZ252Q3DYAFB3', 'ann_marie', 'Ann Marie', 'ann-marie@disco.coop', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FMB775CRJFC9PP87891ZHQNT', 'brian', 'Brian', 'brian@disco.coop', 'Individual');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AH141E8512ZMD23D91NMG', 'disco_mothership', 'Disco Mothership', 'disco-mothership@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AK96JKKJDHTMXM2G43FRA', 'disco_tech_circle', 'Tech Circle', 'tech-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AMH7M3G7A7DCEDK1BC126', 'disco_net_labs_circle', 'NET.LABS Circle', 'net-labs-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8APN9GNE98A6BVZJNH5MBZ', 'disco_diwo_circle', 'DIWO Circle', 'diwo-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8ARC396DTN96EPHVBJ0BQ0', 'disco_leg_fin_circle', 'LEG.FIN Circle', 'leg-fin-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8ASS73J1YZXDS17J25NWSQ', 'disco_stra_dev_circle', 'STA.DEV Circle', 'stra-dev-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AV9GR6SE9WA8C9WXPJ5K7', 'disco_dat_ment_circle', 'DAT.MEN Circle', 'dat-men-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AWJJ71DNYKTZ8MBF90XG1', 'disco_sto_doc_circle', 'STO.DOC Circle', 'sto-doc-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8AYBS82K63N9QERXFZMHTT', 'disco_research_circle', 'RESEARCH Circle', 'research-circle@disco.coop', 'Organization');
INSERT INTO agents (id, unique_name, name, email, agent_type) VALUES ('01FM8B05GFDCPAWGPR0Y7DCKTX', 'disco_community_circle', 'COMMUNITY Circle', 'community-circle@disco.coop', 'Organization');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMCQ5D5KCRD5H02VZCZR2SFT', 'Member of');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMB7SJ3K9CTCJG3NRQWXV37D', 'Commited member of');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMCPZ190W5XB4F5A3Q9GFTSF', 'Dating member of');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMCPZBZB76J486AQYSJD2RDN', 'Steward of');
INSERT INTO agent_relation_types (id, name) VALUES ('01FMCQ3GTVY58JE1XAVEQBSQPM', 'Part of');
/* disco mothership relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMAD6R3FSDQ2GB5HZDESR21Q', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8AH141E8512ZMD23D91NMG', '01FMB7SJ3K9CTCJG3NRQWXV37D');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQAHG4MXX3NJ86YVZ1368T', '01FM8AD71VJ3JG9WRP85XN053M', '01FM8AH141E8512ZMD23D91NMG', '01FMB7SJ3K9CTCJG3NRQWXV37D');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCT1W6ZEZAGFPFKQ450M5T0', '01FM8AEBZT3AKMV0DH98ETPFTR', '01FM8AH141E8512ZMD23D91NMG', '01FMB7SJ3K9CTCJG3NRQWXV37D');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQCARQRN217WS52Y2GRX0V', '01FM8AFR52CMRHZ252Q3DYAFB3', '01FM8AH141E8512ZMD23D91NMG', '01FMB7SJ3K9CTCJG3NRQWXV37D');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCT24GS2MR0XZX4FTY0ASKH', '01FMB775CRJFC9PP87891ZHQNT', '01FM8AH141E8512ZMD23D91NMG', '01FMCPZ190W5XB4F5A3Q9GFTSF');
/* community circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQTQA1PSKB44NDC5E6Q6QB', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8B05GFDCPAWGPR0Y7DCKTX', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQTANT8BEQSN2T1W7QQWRQ', '01FM8AD71VJ3JG9WRP85XN053M', '01FM8B05GFDCPAWGPR0Y7DCKTX', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQTXZ23XPVPWKP36YD7JXH', '01FM8AEBZT3AKMV0DH98ETPFTR', '01FM8B05GFDCPAWGPR0Y7DCKTX', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQV43XDYEEK0G1GSM3YR3Z', '01FM8AFR52CMRHZ252Q3DYAFB3', '01FM8B05GFDCPAWGPR0Y7DCKTX', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCQV9Y6M923X6FGG44XJK9Q', '01FMB775CRJFC9PP87891ZHQNT', '01FM8B05GFDCPAWGPR0Y7DCKTX', '01FMCQ5D5KCRD5H02VZCZR2SFT');
/* DAT.MEN circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCR6MBX7DRG07SYGH4NQFYJ', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8AV9GR6SE9WA8C9WXPJ5K7', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCR6MBXWT27C8R6QC35TV1Q', '01FM8AD71VJ3JG9WRP85XN053M', '01FM8AV9GR6SE9WA8C9WXPJ5K7', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCR6MBXAQ7ER03GTRXVQWW5', '01FM8AEBZT3AKMV0DH98ETPFTR', '01FM8AV9GR6SE9WA8C9WXPJ5K7', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCR6MBXB5XMBTCFDPNGW9E5', '01FM8AFR52CMRHZ252Q3DYAFB3', '01FM8AV9GR6SE9WA8C9WXPJ5K7', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCR6MBXQ18JJNMDBPBY2YYN', '01FMB775CRJFC9PP87891ZHQNT', '01FM8AV9GR6SE9WA8C9WXPJ5K7', '01FMCQ5D5KCRD5H02VZCZR2SFT');
/* NET.LABS circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCS3AAQMY6YKQQNTB8Z4BMV', '01FM8AD71VJ3JG9WRP85XN053M', '01FM8AMH7M3G7A7DCEDK1BC126', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCS3AAQHP8KRKBCYN2BHZ29', '01FM8AEBZT3AKMV0DH98ETPFTR', '01FM8AMH7M3G7A7DCEDK1BC126', '01FMCQ5D5KCRD5H02VZCZR2SFT');
/* DIWO circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCS8PJ57AVKP5ZS3Q6PM8WF', '01FM8AD71VJ3JG9WRP85XN053M', '01FM8APN9GNE98A6BVZJNH5MBZ', '01FMCPZBZB76J486AQYSJD2RDN');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCS8PJ5Q7WNGWKWGNP8TQ65', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8APN9GNE98A6BVZJNH5MBZ', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCS8PJ50TFEEEZ4J892FZQA', '01FM8AFR52CMRHZ252Q3DYAFB3', '01FM8APN9GNE98A6BVZJNH5MBZ', '01FMCQ5D5KCRD5H02VZCZR2SFT');
/* TECH circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSB3JCNBMVGMB8Q3T1SEE5', '01FM8AD71VJ3JG9WRP85XN053M', '01FM8AK96JKKJDHTMXM2G43FRA', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSB3JCR6N3JDGRNQV8CX9J', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8AK96JKKJDHTMXM2G43FRA', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSB3JC6VJ9W39T2Y47AZK0', '01FM8AFR52CMRHZ252Q3DYAFB3', '01FM8AK96JKKJDHTMXM2G43FRA', '01FMCQ5D5KCRD5H02VZCZR2SFT');
/* LEG.FIN circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSDPC4WR21AQVVGERH6AGC', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8ARC396DTN96EPHVBJ0BQ0', '01FMCPZBZB76J486AQYSJD2RDN');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSDPC4BV6RH5V8HZS20QNN', '01FM8AFR52CMRHZ252Q3DYAFB3', '01FM8ARC396DTN96EPHVBJ0BQ0', '01FMCPZBZB76J486AQYSJD2RDN');
/* STO.DOC circle relations */
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSKQN1PRBB3MY82Y9VJX71', '01FM8A9QQBV77N1B1YP7R89RR0', '01FM8AWJJ71DNYKTZ8MBF90XG1', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSKQN16P33QEQ0K81DXWQ0', '01FMB775CRJFC9PP87891ZHQNT', '01FM8AWJJ71DNYKTZ8MBF90XG1', '01FMCQ5D5KCRD5H02VZCZR2SFT');
INSERT INTO agent_relations (id, subject_id, object_id, agent_relation_type_id) VALUES ('01FMCSM56PMQT656XZJ1T86ZZX', '01FM8AEBZT3AKMV0DH98ETPFTR', '01FM8AWJJ71DNYKTZ8MBF90XG1', '01FMCPZBZB76J486AQYSJD2RDN');
