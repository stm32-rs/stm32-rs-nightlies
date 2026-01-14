///Register `RFDCR` reader
pub type R = crate::R<RFDCRrs>;
///Register `RFDCR` writer
pub type W = crate::W<RFDCRrs>;
/**radio debug test bus selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFTBSEL {
    ///0: Digital test bus selected on RF_ADTB\[3:0\]
    Digital = 0,
    ///1: Analog test bus selected on RF_ADTB\[3:0\]
    Analog = 1,
}
impl From<RFTBSEL> for bool {
    #[inline(always)]
    fn from(variant: RFTBSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `RFTBSEL` reader - radio debug test bus selection
pub type RFTBSEL_R = crate::BitReader<RFTBSEL>;
impl RFTBSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFTBSEL {
        match self.bits {
            false => RFTBSEL::Digital,
            true => RFTBSEL::Analog,
        }
    }
    ///Digital test bus selected on RF_ADTB\[3:0\]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == RFTBSEL::Digital
    }
    ///Analog test bus selected on RF_ADTB\[3:0\]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == RFTBSEL::Analog
    }
}
///Field `RFTBSEL` writer - radio debug test bus selection
pub type RFTBSEL_W<'a, REG> = crate::BitWriter<'a, REG, RFTBSEL>;
impl<'a, REG> RFTBSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Digital test bus selected on RF_ADTB\[3:0\]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(RFTBSEL::Digital)
    }
    ///Analog test bus selected on RF_ADTB\[3:0\]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(RFTBSEL::Analog)
    }
}
impl R {
    ///Bit 0 - radio debug test bus selection
    #[inline(always)]
    pub fn rftbsel(&self) -> RFTBSEL_R {
        RFTBSEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFDCR")
            .field("rftbsel", &self.rftbsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - radio debug test bus selection
    #[inline(always)]
    pub fn rftbsel(&mut self) -> RFTBSEL_W<'_, RFDCRrs> {
        RFTBSEL_W::new(self, 0)
    }
}
/**radio debug control register

You can [`read`](crate::Reg::read) this register and get [`rfdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#SYSCFG:RFDCR)*/
pub struct RFDCRrs;
impl crate::RegisterSpec for RFDCRrs {
    type Ux = u32;
}
///`read()` method returns [`rfdcr::R`](R) reader structure
impl crate::Readable for RFDCRrs {}
///`write(|w| ..)` method takes [`rfdcr::W`](W) writer structure
impl crate::Writable for RFDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFDCR to value 0
impl crate::Resettable for RFDCRrs {}
