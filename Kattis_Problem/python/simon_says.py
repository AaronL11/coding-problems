num = int(input())
i = 0
while i < num:
    sentence = input()
    if sentence.startswith("Simon says"):
        sentence = sentence.replace("Simon says ", "")
        print(sentence)
    i += 1