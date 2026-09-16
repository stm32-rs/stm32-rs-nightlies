///Register `DTPR2` reader
pub type R = crate::R<DTPR2rs>;
///Register `DTPR2` writer
pub type W = crate::W<DTPR2rs>;
///Field `TXS` reader - TXS
pub type TXS_R = crate::FieldReader<u16>;
///Field `TXS` writer - TXS
pub type TXS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TXP` reader - TXP
pub type TXP_R = crate::FieldReader;
///Field `TXP` writer - TXP
pub type TXP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TCKE` reader - TCKE
pub type TCKE_R = crate::FieldReader;
///Field `TCKE` writer - TCKE
pub type TCKE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TDLLK` reader - TDLLK
pub type TDLLK_R = crate::FieldReader<u16>;
///Field `TDLLK` writer - TDLLK
pub type TDLLK_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - TXS
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:14 - TXP
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:18 - TCKE
    #[inline(always)]
    pub fn tcke(&self) -> TCKE_R {
        TCKE_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    ///Bits 19:28 - TDLLK
    #[inline(always)]
    pub fn tdllk(&self) -> TDLLK_R {
        TDLLK_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTPR2")
            .field("txs", &self.txs())
            .field("txp", &self.txp())
            .field("tcke", &self.tcke())
            .field("tdllk", &self.tdllk())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - TXS
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W<'_, DTPR2rs> {
        TXS_W::new(self, 0)
    }
    ///Bits 10:14 - TXP
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<'_, DTPR2rs> {
        TXP_W::new(self, 10)
    }
    ///Bits 15:18 - TCKE
    #[inline(always)]
    pub fn tcke(&mut self) -> TCKE_W<'_, DTPR2rs> {
        TCKE_W::new(self, 15)
    }
    ///Bits 19:28 - TDLLK
    #[inline(always)]
    pub fn tdllk(&mut self) -> TDLLK_W<'_, DTPR2rs> {
        TDLLK_W::new(self, 19)
    }
}
/**DDRPHYC DTP register 2

You can [`read`](crate::Reg::read) this register and get [`dtpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:DTPR2)*/
pub struct DTPR2rs;
impl crate::RegisterSpec for DTPR2rs {
    type Ux = u32;
}
///`read()` method returns [`dtpr2::R`](R) reader structure
impl crate::Readable for DTPR2rs {}
///`write(|w| ..)` method takes [`dtpr2::W`](W) writer structure
impl crate::Writable for DTPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTPR2 to value 0x2004_0d84
impl crate::Resettable for DTPR2rs {
    const RESET_VALUE: u32 = 0x2004_0d84;
}
