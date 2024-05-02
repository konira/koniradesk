import { CSSObject, Theme, createTheme } from '@mui/material/styles';
import {  grey , red} from '@mui/material/colors';

// A custom theme for this app
const theme = createTheme({
  palette: {    
    primary: {
      main: '#212121',
    },
    secondary: {
      main: '#204e66',
    },
    background: {
      default: '#242424',
      paper: '#1d1c1c',
    },
    
    text:{
      primary: '#ffffff',
      secondary: '#918f8f',
      disabled: '#000000',      
      },
    error: {
      main: red.A400,
    },    
  },
  typography: {
    fontFamily: [
      'Roboto',
      'sans-serif',
    ].join(','),    
  },
  shape: {
    borderRadius: 10,
  }
});

export default theme;