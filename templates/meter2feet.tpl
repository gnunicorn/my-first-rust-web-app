<html>
<head>
  <style>
    body {
      font-family: Helvetica Neue, Helvetica, sans-serif;
      font-size: 2em;
    }
    #main {
      display: flex;
      width: 55vw;
      height: 100vh;
      margin: 0 auto;
      align-items: center;
      justify-content: center;
    }
  </style>
</head>
<body>
  <div id="main">
    <form method="GET" action="#">
      <label for="meter">Meter:
        <input type="text" id="meter" name="meter" value="{{meter}}" />
      </label>
      <button type="submit"> ➲ </button>
      <label>
        to feet: {{feet}} ft
      </label>
    </form>
  </div>
</body>
</html>
