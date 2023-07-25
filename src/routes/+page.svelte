  <style>
    /* Primary color (Eggplant) */
    :root {
      --primary-color: #52414C;
    }
  
    /* Secondary color (Citron) */
    :root {
      --secondary-color: #CFD186;
    }
  
    main {
      margin: 0;
      padding: 0;
    }
    /* Full-width and full-height container */
    .container {
      display: flex;
      background: linear-gradient(to bottom, #52414C, #362933);
      flex-direction: column;
      align-items: center;
      justify-content: center;
      height: 98.5vh;
      overflow: hidden;
      margin: 0;
      padding: 0;
    }
  
    .title {
      color: var(--secondary-color);
      font-size: 72px;
      font-weight: bold;
      font-family: 'Helvetica', sans-serif;
      text-align: center;
      margin-bottom: 16px;
    }
  
    .description {
      color: var(--secondary-color);
      font-size: 32px;
      font-weight: bold;
      font-family: 'Helvetica', sans-serif;
      text-align: center;
    }
  </style>
  
  <main>
    <div class="container">
      <div class="title">Statusfy</div>
      <div class="description">Your Service Status Tracker</div>
    </div>
  </main>
  