///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPDS` reader - LPDS
pub type LPDS_R = crate::BitReader;
///Field `LPDS` writer - LPDS
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPCFG` reader - LPCFG
pub type LPCFG_R = crate::BitReader;
///Field `LPCFG` writer - LPCFG
pub type LPCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LVDS` reader - LVDS
pub type LVDS_R = crate::BitReader;
///Field `LVDS` writer - LVDS
pub type LVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDEN` reader - PVDEN
pub type PVDEN_R = crate::BitReader;
///Field `PVDEN` writer - PVDEN
pub type PVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - PLS
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - PLS
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBP` reader - DBP
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - DBP
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVDEN` reader - AVDEN
pub type AVDEN_R = crate::BitReader;
///Field `AVDEN` writer - AVDEN
pub type AVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALS` reader - ALS
pub type ALS_R = crate::FieldReader;
///Field `ALS` writer - ALS
pub type ALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - LPDS
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPCFG
    #[inline(always)]
    pub fn lpcfg(&self) -> LPCFG_R {
        LPCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LVDS
    #[inline(always)]
    pub fn lvds(&self) -> LVDS_R {
        LVDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - PVDEN
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - PLS
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - DBP
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - AVDEN
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - ALS
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpds", &self.lpds())
            .field("lpcfg", &self.lpcfg())
            .field("lvds", &self.lvds())
            .field("pvden", &self.pvden())
            .field("pls", &self.pls())
            .field("dbp", &self.dbp())
            .field("avden", &self.avden())
            .field("als", &self.als())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPDS
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CR1rs> {
        LPDS_W::new(self, 0)
    }
    ///Bit 1 - LPCFG
    #[inline(always)]
    pub fn lpcfg(&mut self) -> LPCFG_W<'_, CR1rs> {
        LPCFG_W::new(self, 1)
    }
    ///Bit 2 - LVDS
    #[inline(always)]
    pub fn lvds(&mut self) -> LVDS_W<'_, CR1rs> {
        LVDS_W::new(self, 2)
    }
    ///Bit 4 - PVDEN
    #[inline(always)]
    pub fn pvden(&mut self) -> PVDEN_W<'_, CR1rs> {
        PVDEN_W::new(self, 4)
    }
    ///Bits 5:7 - PLS
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CR1rs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - DBP
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CR1rs> {
        DBP_W::new(self, 8)
    }
    ///Bit 16 - AVDEN
    #[inline(always)]
    pub fn avden(&mut self) -> AVDEN_W<'_, CR1rs> {
        AVDEN_W::new(self, 16)
    }
    ///Bits 17:18 - ALS
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W<'_, CR1rs> {
        ALS_W::new(self, 17)
    }
}
/**Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
