### Tema: Key-Value Storage written in Rust

Operacije:
1. Get - prihvata ključ tipa string (ili niz bajtova), a vraća niz bajtova  
Memtable -> Cache -> Bloom Filter -> SSTable Summary -> SSTable Index -> SSTable Data 
2. Put - prihvata ključ tipa string (ili niz bajtova) i vrednost tipa niz bajtova, a vraća boolean (uspešnost operacije)  
WAL -> Memtable (flush ->) -> SSTable
3. Delete - prihvata ključ tipa string (ili niz bajtova), a vraća boolean (uspešnost operacije), radi se logičko brisanje (kompakcije će kasnije odraditi fizičko brisanje)

Struktura:
1. Write-Ahead Log - služi za čuvanje istorije operacija u slučaju otkaza sistema
2. Memtable - in-memory struktura implementirana preko skip liste koja služi kao nulti nivo LSM stabla i koristi se kao bafer pre zapisivanja na disk
3. Cache - LRU Cache
4. SSTable - kreira se pri pražnjenju bafera (memtable) i sastoji se od sledećih fajlova:
    - Bloom Filter - služi da se sa sigurnošću sazna da li se traženi ključ NE nalazi u toj SSTable
    - Data - sadrži konkretne podatke
    - Index - index svih ključeva sadržanih u Data delu
    - Summary - summary svih ključeva sadržanih u Data delu i početni i kranji ključ sadržanih u Data delu
    - (eventualno) Metadata - Merkle stablo vrednosti u Data delu
5. Token Bucket - služi kao rate limiter, ograničava korišćenje baze na određeni broj operacija u jedinici vremena

Dodatno:
1. LSM kompakcije - kompakcije služe da se više SSTable spoji u jednu veću sa ažuriranim podacima (u tom trenutku bi se odradilo fizičko brisanje).
2. Konfigurisanje - postojala bi eksterna konfiguracija (JSON, YAML ili slično) za parametre struktura i algoritama (broj elemenata u kešu, broj nivoa u LSM stablu, veličina bafera itd.)


Slične implementacije Key-Value baza su LevelDB, RocksDB, Cassandra.