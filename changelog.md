## version/0.0.2
?-?-?
- menambahkan MonagementInit dalam inisialisasi Monagement, MonagementInit memiliki properti antara lain:.
  - start, menjadi awal kategori dan mengatur kategori paling kecil. ukuran berdasarkan pada perpangkatan 2^start
    - contoh: 
      - `start:2`, maka 2^2 maka fl_0 akan memiliki ukuran 4 dengan range 4, 5, 6, 7
      - `start:3`, maka 2^3 maka fl_0 akan memiliki ukuran 8 dengan range 8, 9, 10, 11, 12, 13, 14, 15
  - maximum, menentukan kapasitas maksimal dan free node(space) paling awal di kategorikan.
- total jumlah kategori pada second level kini berdasarkan dari properti start pada MonagementInit, contoh: `start:3`, maka 2^3 = 8, maka akan ada 8 kategori.
- refactoring pada methond `allocated` serta method `free`.
- dalam membebaskan memory yang telah dialokasikan, kini hanya berlaku 2 cara saja:
  - menggunakan method free, `a.free()`
  - menggunakan drop, `drop(a)`
  - untuk pembebasan menggunakan monagement secara langsung tidak dianjurkan.
- update metode bitmap dalam mencari kordinat free node(space). awalnya melakukan linear searching kini langsung melompat ke bit yang aktif tanpa harus melalui bit yang mati. perubahan ini berlaku untuk bitmap yang memetakan first_level dan second_level.
- mengatasi error double free
- pada second_level, link berguna untuk menunjuk kepada free node pada linked_list. algoritma link kini menggunakan linked list alih alih penggunaan array biasa, serta dibantu dengan properti head_link dan end_link untuk langsung menunjuk link paling awal dan link paling akhir.

## version/0.0.1
27-jan-2026
- allocate method.
- free method.
- minimum size is 4.
- the division on the second level is 4 and cannot be changed.
