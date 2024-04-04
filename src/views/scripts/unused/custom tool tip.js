


tooltip: {
    enabled: false,
    position: 'nearest',
    external: externalTooltipHandler
  }
  
const getOrCreateTooltip = (chart) => {
    let tooltipEl = chart.canvas.parentNode.querySelector('div');
  
    if (!tooltipEl) {
      tooltipEl = document.createElement('div');
      tooltipEl.style.background = 'rgba(0, 0, 0, 0.7)';
      tooltipEl.style.borderRadius = '3px';
      tooltipEl.style.color = 'white';
      tooltipEl.style.opacity = 1;
      tooltipEl.style.pointerEvents = 'none';
      tooltipEl.style.position = 'absolute';
      tooltipEl.style.transform = 'translate(-50%, 0)';
      tooltipEl.style.transition = 'all .1s ease';
      tooltipEl.style.width = '200px';
      const table = document.createElement('table');
      table.style.margin = '0px';
  
      tooltipEl.appendChild(table);
      chart.canvas.parentNode.appendChild(tooltipEl);
    }
  
    return tooltipEl;
  };
  
  const externalTooltipHandler = (context) => {
    // Tooltip Element
    const {chart, tooltip} = context;
    const tooltipEl = getOrCreateTooltip(chart);
  
    // Hide if no tooltip
    if (tooltip.opacity === 0) {
      tooltipEl.style.opacity = 0;
      return;
    }
  
    // Set Text
    if (tooltip.body) {
      const titleLines = tooltip.title || [];
      const bodyLines = tooltip.body.map(b => b.lines);
  
      const tableHead = document.createElement('thead');
  
      titleLines.forEach(title => {
        const tr = document.createElement('tr');
        tr.style.borderWidth = 0;
  
        const th = document.createElement('th');
        th.style.borderWidth = 0;
      
        let dataset = context.tooltip.dataPoints[0].dataset;
        let dataIndex = context.tooltip.dataPoints[0].dataIndex;
        //console.log(dataset.tooltipExtraData[dataIndex]);
        const text = document.createTextNode(dataset.tooltipExtraData[dataIndex] + " - " + title + " " );
  
        th.appendChild(text);
        tr.appendChild(th);
        tableHead.appendChild(tr);
      });
  
      const tableBody = document.createElement('tbody');
      bodyLines.forEach((body, i) => {
          const colors = tooltip.labelColors[i];
      
          const span = document.createElement('span');
          span.style.background = colors.backgroundColor;
          span.style.borderColor = colors.borderColor;
          span.style.borderWidth = '2px';
          span.style.marginRight = '10px';
          span.style.height = '10px';
          span.style.width = '10px';
          span.style.display = 'inline-block';

          const span1 = document.createElement('span');
          span1.style.background = colors.backgroundColor;
          span1.style.borderColor = colors.borderColor;
          span1.style.borderWidth = '2px';
          span1.style.marginRight = '10px';
          span1.style.height = '10px';
          span1.style.width = '10px';
          span1.style.display = 'inline-block';

          const span2 = document.createElement('span');
          span2.style.background = colors.backgroundColor;
          span2.style.borderColor = colors.borderColor;
          span2.style.borderWidth = '2px';
          span2.style.marginRight = '10px';
          span2.style.height = '10px';
          span2.style.width = '10px';
          span2.style.display = 'inline-block';
      
          const tr = document.createElement('tr');
          tr.style.backgroundColor = 'inherit';
          tr.style.borderWidth = 0;
      
          const td = document.createElement('td');
          td.style.borderWidth = 0;
      
          const text1 = document.createTextNode(body); // First text node
          const text2 = document.createTextNode("Per Game: "); // Second text node
          const text3 = document.createTextNode("Third text"); // Third text node
      
          td.appendChild(span);
          td.appendChild(text1);
          td.appendChild(document.createElement('br')); // Adding a line break between text nodes
          td.appendChild(span1);
          td.appendChild(text2);
          td.appendChild(document.createElement('br')); // Adding a line break between text nodes
          td.appendChild(span2);
          td.appendChild(text3);
          tr.appendChild(td);
      
          tableBody.appendChild(tr);
      });
  
      const tableRoot = tooltipEl.querySelector('table');
  
      // Remove old children
      while (tableRoot.firstChild) {
        tableRoot.firstChild.remove();
      }
  
      // Add new children
      tableRoot.appendChild(tableHead);
      tableRoot.appendChild(tableBody);
    }
  
    const { offsetLeft: positionX, offsetTop: positionY, offsetWidth: canvasWidth, offsetHeight: canvasHeight } = chart.canvas;
  
 let posY = positionY - tooltipEl.offsetHeight;
  // Display, position, and set styles for font
  tooltipEl.style.opacity = 1;
  tooltipEl.style.left = positionX + tooltip.caretX + 'px';
  tooltipEl.style.top = posY + tooltip.caretY + 'px';
  tooltipEl.style.font = tooltip.options.bodyFont.string;
  tooltipEl.style.padding = tooltip.options.padding + 'px ' + tooltip.options.padding + 'px';
  };