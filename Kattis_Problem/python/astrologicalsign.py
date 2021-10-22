# Solved

months = {
    'Mar': {
        20: 'Pisces',
        21: 'Aries'
    },
    'Apr': {
        20: 'Aries',
        21: 'Taurus'
    },
    'May': {
        20: 'Taurus',
        21: 'Gemini'
    },
    'Jun': {
        21: 'Gemini',
        22: 'Cancer'
    },
    'Jul': {
        22: 'Cancer',
        23: 'Leo'
    },
    'Aug': {
        22: 'Leo',
        23: 'Virgo'
    },
    'Sep': {
        21: 'Virgo',
        22: 'Libra'
    },
    'Oct': {
        22: 'Libra',
        23: 'Scorpio'
    },
    'Nov': {
        22: 'Scorpio',
        23: 'Sagittarius'
    },
    'Dec': {
        21: 'Sagittarius',
        22: 'Capricorn'
    },
    'Jan': {
        20: 'Capricorn',
        21: 'Aquarius'
    },
    'Feb': {
        19: 'Aquarius',
        20: 'Pisces'
    }
}


t = int(input())

for _ in range(t):
    d,m = input().split()
    d = int(d)
    for i,date in enumerate(months[m]):
        if i==0 and d<=date:
            print(months[m][date])
            break
        elif i==1 and d>=date:
            print(months[m][date])
            break




