# next p = 42

p=0.1

for _ in {0..9}
do
	cargo run ../../files/pan-tadeusz-czyli-ostatni-zajazd-na-litwie.txt $p >> result.txt
	p=$(bc <<< $(echo $p + 0.1))
done
