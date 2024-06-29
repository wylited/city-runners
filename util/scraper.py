import webbrowser

def open_mtr_links():
    base_url = "https://www.mtr.com.hk/en/customer/jp/index.php?"
    inputs = []
    while True:
        val = input("('q' to quit): \n")
        if val.lower() == 'q':
            break
        inputs.append(int(val))
    for i in range(len(inputs) - 1):
        o_value = inputs[i]
        d_value = inputs[i + 1]
        url = f"{base_url}oValue={o_value}&dValue={d_value}"
        webbrowser.open(url)

open_mtr_links()
