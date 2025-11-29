// src/app/ClientPage.tsx
'use client';

import { useState, useEffect } from 'react';
import { useTranslations } from 'next-intl';
import Link from 'next/link';

// Componente principal de la página
export default function ClientPage({ locale, messages }: { locale: string; messages: any }) {
  const t = useTranslations('IndexPage');
  const [timeLeft, setTimeLeft] = useState({
    days: 0,
    hours: 0,
    minutes: 0,
    seconds: 0
  });

  // Fecha del testnet (Q2 2026)
  const targetDate = new Date('2026-06-01T00:00:00').getTime();

  // Efecto para calcular el tiempo restante
  useEffect(() => {
    const calculateTimeLeft = () => {
      const now = new Date().getTime();
      const difference = targetDate - now;

      if (difference > 0) {
        const days = Math.floor(difference / (1000 * 60 * 60 * 24));
        const hours = Math.floor((difference % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
        const minutes = Math.floor((difference % (1000 * 60 * 60)) / (1000 * 60));
        const seconds = Math.floor((difference % (1000 * 60)) / 1000);

        setTimeLeft({ days, hours, minutes, seconds });
      }
    };

    // Calcular inmediatamente
    calculateTimeLeft();

    // Actualizar cada segundo
    const timer = setInterval(calculateTimeLeft, 1000);

    // Limpiar intervalo al desmontar
    return () => clearInterval(timer);
  }, []);

  // Función para cambiar idioma
  const changeLanguage = (newLocale: string) => {
    // Actualizar la URL con el nuevo idioma
    window.location.href = `/${newLocale}`;
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 to-blue-900 text-white">
      {/* Barra de navegación con selector de idioma */}
      <nav className="p-4 flex justify-between items-center bg-gray-800/50 backdrop-blur-sm">
        <div className="text-xl font-bold">QubitCoin</div>
        <div className="flex items-center space-x-4">
          <span className="text-sm">{t('languageSwitcher')}:</span>
          <select 
            value={locale}
            onChange={(e) => changeLanguage(e.target.value)}
            className="bg-gray-700 text-white px-3 py-1 rounded"
          >
            <option value="es">ES</option>
            <option value="en">EN</option>
            <option value="de">DE</option>
          </select>
        </div>
      </nav>

      {/* Sección Hero */}
      <section className="py-20 px-4 text-center">
        <div className="max-w-4xl mx-auto">
          <h1 className="text-4xl md:text-6xl font-bold mb-6">
            {t('heroTitle')}
          </h1>
          <p className="text-xl md:text-2xl mb-10 text-gray-300">
            {t('heroSubtitle')}
          </p>
          <div className="flex flex-wrap justify-center gap-4 mb-8">
            <a
              href="https://raw.githubusercontent.com/RaulObsidian/QubitCoin-Core/main/whitepaper/QubitCoin_Whitepaper_v1.0.pdf"
              download="QubitCoin_Whitepaper_Completo_ES.pdf"
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-bold py-3 px-6 rounded-full transition duration-300 shadow-lg"
            >
              Descargar Whitepaper Completo (Español)
            </a>
            <a
              href="https://raw.githubusercontent.com/RaulObsidian/QubitCoin-Core/main/whitepaper/QubitCoin_Whitepaper_v1.0_EN.pdf"
              download="QubitCoin_Whitepaper_Complete_EN.pdf"
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-bold py-3 px-6 rounded-full transition duration-300 shadow-lg"
            >
              Download Complete Whitepaper (English)
            </a>
            <a
              href="https://raw.githubusercontent.com/RaulObsidian/QubitCoin-Core/main/whitepaper/QubitCoin_Whitepaper_v1.0_DE.pdf"
              download="QubitCoin_Whitepaper_Komplett_DE.pdf"
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-bold py-3 px-6 rounded-full transition duration-300 shadow-lg"
            >
              Komplettes Whitepaper herunterladen (Deutsch)
            </a>
          </div>
        </div>
      </section>

      {/* Sección de Cuenta Regresiva */}
      <section className="py-16 px-4 bg-gray-800/30">
        <div className="max-w-6xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-4">{t('countdownTitle')}</h2>
          <p className="text-lg mb-10 text-gray-300">{t('countdownSubtitle')}</p>
          
          <div className="flex justify-center space-x-4 md:space-x-8">
            <div className="text-center">
              <div className="text-4xl md:text-6xl font-bold text-blue-400">{timeLeft.days}</div>
              <div className="text-sm md:text-base">{t('days')}</div>
            </div>
            <div className="text-center">
              <div className="text-4xl md:text-6xl font-bold text-blue-400">{timeLeft.hours}</div>
              <div className="text-sm md:text-base">{t('hours')}</div>
            </div>
            <div className="text-center">
              <div className="text-4xl md:text-6xl font-bold text-blue-400">{timeLeft.minutes}</div>
              <div className="text-sm md:text-base">{t('minutes')}</div>
            </div>
            <div className="text-center">
              <div className="text-4xl md:text-6xl font-bold text-blue-400">{timeLeft.seconds}</div>
              <div className="text-sm md:text-base">{t('seconds')}</div>
            </div>
          </div>
        </div>
      </section>

      {/* Sección de Tokenómica */}
      <section className="py-16 px-4">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl font-bold mb-4 text-center">{t('tokenomicsTitle')}</h2>
          <p className="text-lg mb-12 text-gray-300 text-center">{t('tokenomicsSubtitle')}</p>
          
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div className="bg-gray-800/50 p-6 rounded-lg text-center">
              <div className="text-3xl font-bold text-blue-400">21M</div>
              <div className="mt-2">{t('totalSupply')}</div>
            </div>
            <div className="bg-gray-800/50 p-6 rounded-lg text-center">
              <div className="text-3xl font-bold text-green-400">14.7M</div>
              <div className="mt-2">{t('miningRewards')}</div>
            </div>
            <div className="bg-gray-800/50 p-6 rounded-lg text-center">
              <div className="text-3xl font-bold text-yellow-400">4.2M</div>
              <div className="mt-2">{t('devCommunity')}</div>
            </div>
            <div className="bg-gray-800/50 p-6 rounded-lg text-center">
              <div className="text-3xl font-bold text-purple-400">2.1M</div>
              <div className="mt-2">{t('foundersInvestors')}</div>
            </div>
          </div>
        </div>
      </section>

      {/* Sección de Hoja de Ruta */}
      <section className="py-16 px-4 bg-gray-800/30">
        <div className="max-w-6xl mx-auto">
          <h2 className="text-3xl font-bold mb-12 text-center">{t('roadmapTitle')}</h2>
          
          <div className="space-y-8">
            <div className="flex flex-col md:flex-row items-start md:items-center">
              <div className="md:w-1/4 mb-4 md:mb-0 font-bold">{t('q4_2025')}</div>
              <div className="md:w-3/4 bg-gray-700/50 p-4 rounded-lg">
                {t('whitepaperLaunch')}
              </div>
            </div>
            
            <div className="flex flex-col md:flex-row items-start md:items-center">
              <div className="md:w-1/4 mb-4 md:mb-0 font-bold">{t('q1_2026')}</div>
              <div className="md:w-3/4 bg-gray-700/50 p-4 rounded-lg">
                {t('publicTestnet')}
              </div>
            </div>
            
            <div className="flex flex-col md:flex-row items-start md:items-center">
              <div className="md:w-1/4 mb-4 md:mb-0 font-bold">{t('q2_2026')}</div>
              <div className="md:w-3/4 bg-gray-700/50 p-4 rounded-lg">
                {t('mainnetGenesis')}
              </div>
            </div>
            
            <div className="flex flex-col md:flex-row items-start md:items-center">
              <div className="md:w-1/4 mb-4 md:mb-0 font-bold">{t('q4_2026')}</div>
              <div className="md:w-3/4 bg-gray-700/50 p-4 rounded-lg">
                {t('smartContracts')}
              </div>
            </div>
            
            <div className="flex flex-col md:flex-row items-start md:items-center">
              <div className="md:w-1/4 mb-4 md:mb-0 font-bold">{t('q2_2027')}</div>
              <div className="md:w-3/4 bg-gray-700/50 p-4 rounded-lg">
                {t('scalability')}
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Sección de Descargas */}
      <section className="py-16 px-4">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-8">{t('downloadDocuments')}</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <a
              href="https://raw.githubusercontent.com/RaulObsidian/QubitCoin-Core/main/whitepaper/QubitCoin_Whitepaper_v1.0.pdf"
              download="QubitCoin_Whitepaper_Completo_ES.pdf"
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 p-6 rounded-lg transition duration-300 shadow-lg"
            >
              <h3 className="text-xl font-bold mb-2">Descargar Whitepaper Completo</h3>
              <p className="text-sm">Español - PDF</p>
            </a>

            <a
              href="https://raw.githubusercontent.com/RaulObsidian/QubitCoin-Core/main/whitepaper/QubitCoin_Whitepaper_v1.0_EN.pdf"
              download="QubitCoin_Whitepaper_Complete_EN.pdf"
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 p-6 rounded-lg transition duration-300 shadow-lg"
            >
              <h3 className="text-xl font-bold mb-2">Download Complete Whitepaper</h3>
              <p className="text-sm">English - PDF</p>
            </a>

            <a
              href="https://raw.githubusercontent.com/RaulObsidian/QubitCoin-Core/main/whitepaper/QubitCoin_Whitepaper_v1.0_DE.pdf"
              download="QubitCoin_Whitepaper_Komplett_DE.pdf"
              className="bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 p-6 rounded-lg transition duration-300 shadow-lg"
            >
              <h3 className="text-xl font-bold mb-2">Komplettes Whitepaper herunterladen</h3>
              <p className="text-sm">Deutsch - PDF</p>
            </a>

            <a
              href="/es/Pitch_Deck_QubitCoin_ES.pdf"
              target="_blank"
              className="bg-blue-700 hover:bg-blue-800 p-6 rounded-lg transition duration-300"
            >
              <h3 className="text-xl font-bold mb-2">{t('downloadPitchDeck')}</h3>
              <p className="text-sm">Versión en español</p>
            </a>

            <a
              href="/en/Pitch_Deck_QubitCoin_EN.pdf"
              target="_blank"
              className="bg-blue-700 hover:bg-blue-800 p-6 rounded-lg transition duration-300"
            >
              <h3 className="text-xl font-bold mb-2">{t('downloadPitchDeck')}</h3>
              <p className="text-sm">English Version</p>
            </a>

            <a
              href="/de/Pitch_Deck_QubitCoin_DE.pdf"
              target="_blank"
              className="bg-blue-700 hover:bg-blue-800 p-6 rounded-lg transition duration-300"
            >
              <h3 className="text-xl font-bold mb-2">{t('downloadPitchDeck')}</h3>
              <p className="text-sm">Deutsche Version</p>
            </a>

            <a
              href="/es/Dossier_Inversores_v2_ES.pdf"
              target="_blank"
              className="bg-green-700 hover:bg-green-800 p-6 rounded-lg transition duration-300"
            >
              <h3 className="text-xl font-bold mb-2">{t('downloadDossier')}</h3>
              <p className="text-sm">Versión en español</p>
            </a>

            <a
              href="/en/Dossier_Inversores_v2_EN.pdf"
              target="_blank"
              className="bg-green-700 hover:bg-green-800 p-6 rounded-lg transition duration-300"
            >
              <h3 className="text-xl font-bold mb-2">{t('downloadDossier')}</h3>
              <p className="text-sm">English Version</p>
            </a>

            <a
              href="/de/Dossier_Inversores_v2_DE.pdf"
              target="_blank"
              className="bg-green-700 hover:bg-green-800 p-6 rounded-lg transition duration-300"
            >
              <h3 className="text-xl font-bold mb-2">{t('downloadDossier')}</h3>
              <p className="text-sm">Deutsche Version</p>
            </a>
          </div>
        </div>
      </section>

      {/* Pie de página */}
      <footer className="py-8 px-4 bg-gray-900 text-center text-gray-400">
        <p>© {new Date().getFullYear()} QubitCoin. Todos los derechos reservados.</p>
      </footer>
    </div>
  );
}