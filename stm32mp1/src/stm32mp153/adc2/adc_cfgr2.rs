///Register `ADC_CFGR2` reader
pub type R = crate::R<ADC_CFGR2rs>;
///Register `ADC_CFGR2` writer
pub type W = crate::W<ADC_CFGR2rs>;
///Field `ROVSE` reader - ROVSE
pub type ROVSE_R = crate::BitReader;
///Field `ROVSE` writer - ROVSE
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVSE` reader - JOVSE
pub type JOVSE_R = crate::BitReader;
///Field `JOVSE` writer - JOVSE
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVSS` reader - OVSS
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - OVSS
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TROVS` reader - TROVS
pub type TROVS_R = crate::BitReader;
///Field `TROVS` writer - TROVS
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVSM` reader - ROVSM
pub type ROVSM_R = crate::BitReader;
///Field `ROVSM` writer - ROVSM
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSHIFT1` reader - RSHIFT1
pub type RSHIFT1_R = crate::BitReader;
///Field `RSHIFT1` writer - RSHIFT1
pub type RSHIFT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSHIFT2` reader - RSHIFT2
pub type RSHIFT2_R = crate::BitReader;
///Field `RSHIFT2` writer - RSHIFT2
pub type RSHIFT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSHIFT3` reader - RSHIFT3
pub type RSHIFT3_R = crate::BitReader;
///Field `RSHIFT3` writer - RSHIFT3
pub type RSHIFT3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSHIFT4` reader - RSHIFT4
pub type RSHIFT4_R = crate::BitReader;
///Field `RSHIFT4` writer - RSHIFT4
pub type RSHIFT4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSVR` reader - OSVR
pub type OSVR_R = crate::FieldReader<u16>;
///Field `OSVR` writer - OSVR
pub type OSVR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `LSHIFT` reader - LSHIFT
pub type LSHIFT_R = crate::FieldReader;
///Field `LSHIFT` writer - LSHIFT
pub type LSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - ROVSE
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - JOVSE
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - TROVS
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ROVSM
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RSHIFT1
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT1_R {
        RSHIFT1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RSHIFT2
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT2_R {
        RSHIFT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RSHIFT3
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT3_R {
        RSHIFT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RSHIFT4
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT4_R {
        RSHIFT4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:25 - OSVR
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 28:31 - LSHIFT
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR2")
            .field("rovse", &self.rovse())
            .field("jovse", &self.jovse())
            .field("ovss", &self.ovss())
            .field("trovs", &self.trovs())
            .field("rovsm", &self.rovsm())
            .field("rshift1", &self.rshift1())
            .field("rshift2", &self.rshift2())
            .field("rshift3", &self.rshift3())
            .field("rshift4", &self.rshift4())
            .field("osvr", &self.osvr())
            .field("lshift", &self.lshift())
            .finish()
    }
}
impl W {
    ///Bit 0 - ROVSE
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<ADC_CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - JOVSE
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<ADC_CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 5:8 - OVSS
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<ADC_CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - TROVS
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<ADC_CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - ROVSM
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<ADC_CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    ///Bit 11 - RSHIFT1
    #[inline(always)]
    #[must_use]
    pub fn rshift1(&mut self) -> RSHIFT1_W<ADC_CFGR2rs> {
        RSHIFT1_W::new(self, 11)
    }
    ///Bit 12 - RSHIFT2
    #[inline(always)]
    #[must_use]
    pub fn rshift2(&mut self) -> RSHIFT2_W<ADC_CFGR2rs> {
        RSHIFT2_W::new(self, 12)
    }
    ///Bit 13 - RSHIFT3
    #[inline(always)]
    #[must_use]
    pub fn rshift3(&mut self) -> RSHIFT3_W<ADC_CFGR2rs> {
        RSHIFT3_W::new(self, 13)
    }
    ///Bit 14 - RSHIFT4
    #[inline(always)]
    #[must_use]
    pub fn rshift4(&mut self) -> RSHIFT4_W<ADC_CFGR2rs> {
        RSHIFT4_W::new(self, 14)
    }
    ///Bits 16:25 - OSVR
    #[inline(always)]
    #[must_use]
    pub fn osvr(&mut self) -> OSVR_W<ADC_CFGR2rs> {
        OSVR_W::new(self, 16)
    }
    ///Bits 28:31 - LSHIFT
    #[inline(always)]
    #[must_use]
    pub fn lshift(&mut self) -> LSHIFT_W<ADC_CFGR2rs> {
        LSHIFT_W::new(self, 28)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC2:ADC_CFGR2)*/
pub struct ADC_CFGR2rs;
impl crate::RegisterSpec for ADC_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfgr2::R`](R) reader structure
impl crate::Readable for ADC_CFGR2rs {}
///`write(|w| ..)` method takes [`adc_cfgr2::W`](W) writer structure
impl crate::Writable for ADC_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CFGR2 to value 0
impl crate::Resettable for ADC_CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
