async function generateGuaXiang() {
    const input = document.getElementById('yaoInput').value;
    if (input.length !== 6) {
        alert("请输入6位数字（0-3）");
        return;
    }

    const response = await fetch('/generate_gua_xian', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ numbers: input }),
    });

    if (response.ok) {
        const data = await response.json();
        const guaDiagram = document.getElementById('guaDiagram');
        guaDiagram.innerHTML = '';

        data.gua_xian.reverse().forEach(gua => {
            const div = document.createElement('div');
            div.textContent = gua;
            guaDiagram.appendChild(div);
        });

        document.getElementById('result').classList.add('show');
    } else {
        alert("生成卦象失败，请重试");
    }
}