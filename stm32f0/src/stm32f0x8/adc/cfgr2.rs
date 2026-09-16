///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**ADC clock mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    ///0: Asynchronous clock mode
    Adcclk = 0,
    ///1: Synchronous clock mode (PCLK/2)
    PclkDiv2 = 1,
    ///2: Sychronous clock mode (PCLK/4)
    PclkDiv4 = 2,
}
impl From<CKMODE> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE {
    type Ux = u8;
}
impl crate::IsEnum for CKMODE {}
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKMODE> {
        match self.bits {
            0 => Some(CKMODE::Adcclk),
            1 => Some(CKMODE::PclkDiv2),
            2 => Some(CKMODE::PclkDiv4),
            _ => None,
        }
    }
    ///Asynchronous clock mode
    #[inline(always)]
    pub fn is_adcclk(&self) -> bool {
        *self == CKMODE::Adcclk
    }
    ///Synchronous clock mode (PCLK/2)
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE::PclkDiv2
    }
    ///Sychronous clock mode (PCLK/4)
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE::PclkDiv4
    }
}
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Asynchronous clock mode
    #[inline(always)]
    pub fn adcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Adcclk)
    }
    ///Synchronous clock mode (PCLK/2)
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv2)
    }
    ///Sychronous clock mode (PCLK/4)
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv4)
    }
}
impl R {
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("ckmode", &self.ckmode())
            .finish()
    }
}
impl W {
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
/**configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#ADC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
