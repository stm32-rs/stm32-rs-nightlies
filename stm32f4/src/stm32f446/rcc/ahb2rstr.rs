///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**Camera interface reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<DCMIRST> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMIRST` reader - Camera interface reset
pub type DCMIRST_R = crate::BitReader<DCMIRST>;
impl DCMIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCMIRST> {
        match self.bits {
            true => Some(DCMIRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMIRST::Reset
    }
}
///Field `DCMIRST` writer - Camera interface reset
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG, DCMIRST>;
impl<'a, REG> DCMIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIRST::Reset)
    }
}
///Field `OTGFSRST` reader - USB OTG FS module reset
pub use DCMIRST_R as OTGFSRST_R;
///Field `OTGFSRST` writer - USB OTG FS module reset
pub use DCMIRST_W as OTGFSRST_W;
impl R {
    ///Bit 0 - Camera interface reset
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("dcmirst", &self.dcmirst())
            .field("otgfsrst", &self.otgfsrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Camera interface reset
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W<'_, AHB2RSTRrs> {
        DCMIRST_W::new(self, 0)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB2RSTRrs> {
        OTGFSRST_W::new(self, 7)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
