### Tema: Key-Value Storage written in Rust

#### Struktura:
1. Write-Ahead Log - Služi za čuvanje istorije operacija u slučaju otkaza sistema. Implementira se kao segmentirani log što znači da će se na određeni broj upisa (ili na osnovu veličine segmenta) napraviti novi segment (fajl).

Primer kako bi mogao da izgleda jedan upis u WAL (i upis u Data deo SSTable):
![Primer jednog upisa u WAL](https://i.imgur.com/Z95BVy4.png)

2. Memtable - In-memory struktura implementirana preko Skip liste koja služi kao nulti nivo LSM stabla i koristi se kao bafer pre zapisivanja na disk.
3. Cache - Keš implementiran preko preko LRU strategije.
4. SSTable - Kreira se pri pražnjenju bafera (memtable) i sastoji se od sledećih fajlova:
    - Bloom Filter - Služi da se sa sigurnošću sazna da li se traženi ključ NE nalazi u toj SSTable.
    - Data - Sadrži konkretne podatke.
    - Index - Za svaki ključ iz Data dela je zapisan njegov offset u Data delu radi bržeg pristupa.
    - Summary - Summary svih ključeva sadržanih u Data delu i početni i kranji ključ sadržanih u Data delu.
    - (eventualno) Metadata - Merkle stablo vrednosti u Data delu koje služi za proveru da li su dva Data dela ista (npr. u slučaju da se SSTable ili samo Data deo prenese preko mreže, da se na drugoj strani može proveriti da li je validan fajl stigao).
5. Token Bucket - Služi kao rate limiter, ograničava korišćenje baze na određeni broj operacija u jedinici vremena.

#### Operacije:
1. Get - prihvata ključ tipa string (ili niz bajtova), a vraća niz bajtova  

![Putanja za Get operaciju](https://i.imgur.com/ZI6TYKe.png)

2. Put - prihvata ključ tipa string (ili niz bajtova) i vrednost tipa niz bajtova, a vraća boolean (uspešnost operacije)  

![Putanja za Put operaciju](https://i.imgur.com/6kRSy3G.png)

3. Delete - prihvata ključ tipa string (ili niz bajtova), a vraća boolean (uspešnost operacije), radi se logičko brisanje (kompakcije će kasnije odraditi fizičko brisanje)

#### Dodatno:
1. LSM kompakcije - Kompakcije služe da se više SSTable spoji u jednu veću sa ažuriranim podacima (u tom trenutku bi se odradilo fizičko brisanje).
2. Konfigurisanje - Postojala bi eksterna konfiguracija (JSON, YAML ili slično) za parametre struktura i algoritama (broj elemenata u kešu, broj nivoa u LSM stablu, veličina bafera itd.).

#### GUI:  
Aplikacija je zamišljena kao bibliotečka aplikacija. U svrhu testiranja može posedovati CLI.

Slične implementacije Key-Value baza su LevelDB, RocksDB, Cassandra.
https://artem.krylysov.com/blog/2023/04/19/how-rocksdb-works/