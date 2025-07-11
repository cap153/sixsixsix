// 页面加载完成后，立即生成地支掌诀图
document.addEventListener('DOMContentLoaded', () => {
    createDizhiGrid();
});

function addWuxingColorClass(element, branchChar) {
    // 移除所有可能的五行类，确保清洁
    element.classList.remove('earth', 'wood', 'fire', 'metal', 'water');
    
    switch (branchChar) {
        case '子': case '亥':
            element.classList.add('water');
            break;
        case '寅': case '卯':
            element.classList.add('wood');
            break;
        case '巳': case '午':
            element.classList.add('fire');
            break;
        case '申': case '酉':
            element.classList.add('metal');
            break;
        case '辰': case '戌': case '丑': case '未':
            element.classList.add('earth');
            break;
    }
}

async function generateGuaXiang() {
    const input = document.getElementById('yaoInput').value;
    if (input.length !== 6) {
        alert("请输入6位数字（0-3）");
        return;
    }

    const button = document.querySelector('button');
    button.textContent = '推演中...';
    button.disabled = true;

    try {
        const response = await fetch('/generate_gua_xian', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ numbers: input }),
        });

        if (response.ok) {
            const data = await response.json();
            const resultContainer = document.getElementById('result');
            resultContainer.innerHTML = ''; // 清空旧内容

            // 创建并展示干支信息
            const ganzhiInfoDiv = document.createElement('div');
            ganzhiInfoDiv.className = 'ganzhi-info';
            const ganzhiParts = [
                { text: data.year_ganzhi + '年', branchChar: data.year_ganzhi.charAt(1) },
                { text: data.month_ganzhi + '月', branchChar: data.month_ganzhi.charAt(1) },
                { text: data.day_ganzhi + '日', branchChar: data.day_ganzhi.charAt(1) },
                { text: data.hour_ganzhi + '时', branchChar: data.hour_ganzhi.charAt(0) }
            ];
            ganzhiParts.forEach(part => {
                const span = document.createElement('span');
                span.textContent = part.text;
                addWuxingColorClass(span, part.branchChar);
                ganzhiInfoDiv.appendChild(span);
            });
            resultContainer.appendChild(ganzhiInfoDiv);

            // 创建卦象展示网格
            const guaDisplayGrid = document.createElement('div');
            guaDisplayGrid.className = 'gua-display-grid';

            // 反转数组以正确顺序显示 (初爻在下，上爻在上)
            data.gua_lines.reverse().forEach((lineData, index) => {
                const isGuaNameRow = (index === 0);

                // 正卦容器
                const zhengDiv = document.createElement('div');
                zhengDiv.className = 'gua-line zheng-gua';
                
                // 变卦容器
                const bianDiv = document.createElement('div');
                bianDiv.className = 'gua-line bian-gua';
                
                if (isGuaNameRow) {
                    zhengDiv.classList.add('gua-name');
                    zhengDiv.textContent = lineData.base_text;
                    bianDiv.classList.add('gua-name');
                    bianDiv.textContent = lineData.bian_text;
                } else {
                    // 1. 添加基础文本 (六亲、地支、五行、爻象)
                    const baseSpan = document.createElement('span');
                    baseSpan.textContent = lineData.base_text;
                    addWuxingColorClass(baseSpan, lineData.base_text.charAt(2));
                    zhengDiv.appendChild(baseSpan);

                    // 2. 如果是世或应，创建独立的、带样式的 <span>
                    if (lineData.role === 'Shi' || lineData.role === 'Ying') {
                        const roleSpan = document.createElement('span');
                        roleSpan.className = `role-tag ${lineData.role.toLowerCase()}-tag`;
                        roleSpan.textContent = lineData.role === 'Shi' ? '世' : '应';
                        zhengDiv.appendChild(roleSpan);
                    }

                    // 3. 添加关系文本
                    const relationSpan = document.createElement('span');
                    relationSpan.className = 'relation-text';
                    relationSpan.textContent = lineData.relations_text;
                    zhengDiv.appendChild(relationSpan);
                    
                    // 变卦内容
                    const bianBaseSpan = document.createElement('span');
                    bianBaseSpan.textContent = lineData.bian_text;
                    addWuxingColorClass(bianBaseSpan, lineData.bian_text.charAt(2));
                    bianDiv.appendChild(bianBaseSpan);
                }

                guaDisplayGrid.appendChild(zhengDiv);
                guaDisplayGrid.appendChild(bianDiv);
            });
            
            resultContainer.appendChild(guaDisplayGrid);
            resultContainer.classList.add('show');

        } else {
            alert("生成卦象失败，请重试");
        }
    } catch (error) {
        console.error('Error:', error);
        alert('网络请求失败，请检查后端服务是否运行。');
    } finally {
        button.textContent = '推演卦象';
        button.disabled = false;
    }
}

function createDizhiGrid() {
    const gridContainer = document.querySelector('.dizhi-grid');
    if (!gridContainer) return;

    const layout = [
        ['巳', '午', '未', '申'],
        ['辰',  '',   '',  '酉'],
        ['卯',  '',   '',  '戌'],
        ['寅', '丑', '子', '亥']
    ];

    layout.forEach(row => {
        row.forEach(char => {
            const cell = document.createElement('div');
            cell.className = 'dizhi-cell';
            if (char) {
                cell.textContent = char;
                addWuxingColorClass(cell, char);
            } else {
                cell.classList.add('empty');
            }
            gridContainer.appendChild(cell);
        });
    });
}
