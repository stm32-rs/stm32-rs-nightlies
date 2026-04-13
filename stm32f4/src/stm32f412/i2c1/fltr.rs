///Register `FLTR` reader
pub type R = crate::R<FLTRrs>;
///Register `FLTR` writer
pub type W = crate::W<FLTRrs>;
/**Digital noise filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF {
    ///0: Digital filter disabled
    NoFilter = 0,
    ///1: Digital filter enabled and filtering capability up to 1 tI2CCLK
    Filter1 = 1,
    ///2: Digital filter enabled and filtering capability up to 2 tI2CCLK
    Filter2 = 2,
    ///3: Digital filter enabled and filtering capability up to 3 tI2CCLK
    Filter3 = 3,
    ///4: Digital filter enabled and filtering capability up to 4 tI2CCLK
    Filter4 = 4,
    ///5: Digital filter enabled and filtering capability up to 5 tI2CCLK
    Filter5 = 5,
    ///6: Digital filter enabled and filtering capability up to 6 tI2CCLK
    Filter6 = 6,
    ///7: Digital filter enabled and filtering capability up to 7 tI2CCLK
    Filter7 = 7,
    ///8: Digital filter enabled and filtering capability up to 8 tI2CCLK
    Filter8 = 8,
    ///9: Digital filter enabled and filtering capability up to 9 tI2CCLK
    Filter9 = 9,
    ///10: Digital filter enabled and filtering capability up to 10 tI2CCLK
    Filter10 = 10,
    ///11: Digital filter enabled and filtering capability up to 11 tI2CCLK
    Filter11 = 11,
    ///12: Digital filter enabled and filtering capability up to 12 tI2CCLK
    Filter12 = 12,
    ///13: Digital filter enabled and filtering capability up to 13 tI2CCLK
    Filter13 = 13,
    ///14: Digital filter enabled and filtering capability up to 14 tI2CCLK
    Filter14 = 14,
    ///15: Digital filter enabled and filtering capability up to 15 tI2CCLK
    Filter15 = 15,
}
impl From<DNF> for u8 {
    #[inline(always)]
    fn from(variant: DNF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DNF {
    type Ux = u8;
}
impl crate::IsEnum for DNF {}
///Field `DNF` reader - Digital noise filter
pub type DNF_R = crate::FieldReader<DNF>;
impl DNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DNF {
        match self.bits {
            0 => DNF::NoFilter,
            1 => DNF::Filter1,
            2 => DNF::Filter2,
            3 => DNF::Filter3,
            4 => DNF::Filter4,
            5 => DNF::Filter5,
            6 => DNF::Filter6,
            7 => DNF::Filter7,
            8 => DNF::Filter8,
            9 => DNF::Filter9,
            10 => DNF::Filter10,
            11 => DNF::Filter11,
            12 => DNF::Filter12,
            13 => DNF::Filter13,
            14 => DNF::Filter14,
            15 => DNF::Filter15,
            _ => unreachable!(),
        }
    }
    ///Digital filter disabled
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF::NoFilter
    }
    ///Digital filter enabled and filtering capability up to 1 tI2CCLK
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF::Filter1
    }
    ///Digital filter enabled and filtering capability up to 2 tI2CCLK
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF::Filter2
    }
    ///Digital filter enabled and filtering capability up to 3 tI2CCLK
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF::Filter3
    }
    ///Digital filter enabled and filtering capability up to 4 tI2CCLK
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF::Filter4
    }
    ///Digital filter enabled and filtering capability up to 5 tI2CCLK
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF::Filter5
    }
    ///Digital filter enabled and filtering capability up to 6 tI2CCLK
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF::Filter6
    }
    ///Digital filter enabled and filtering capability up to 7 tI2CCLK
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF::Filter7
    }
    ///Digital filter enabled and filtering capability up to 8 tI2CCLK
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF::Filter8
    }
    ///Digital filter enabled and filtering capability up to 9 tI2CCLK
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF::Filter9
    }
    ///Digital filter enabled and filtering capability up to 10 tI2CCLK
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF::Filter10
    }
    ///Digital filter enabled and filtering capability up to 11 tI2CCLK
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF::Filter11
    }
    ///Digital filter enabled and filtering capability up to 12 tI2CCLK
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF::Filter12
    }
    ///Digital filter enabled and filtering capability up to 13 tI2CCLK
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF::Filter13
    }
    ///Digital filter enabled and filtering capability up to 14 tI2CCLK
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF::Filter14
    }
    ///Digital filter enabled and filtering capability up to 15 tI2CCLK
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF::Filter15
    }
}
///Field `DNF` writer - Digital noise filter
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DNF, crate::Safe>;
impl<'a, REG> DNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Digital filter disabled
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::NoFilter)
    }
    ///Digital filter enabled and filtering capability up to 1 tI2CCLK
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter1)
    }
    ///Digital filter enabled and filtering capability up to 2 tI2CCLK
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter2)
    }
    ///Digital filter enabled and filtering capability up to 3 tI2CCLK
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter3)
    }
    ///Digital filter enabled and filtering capability up to 4 tI2CCLK
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter4)
    }
    ///Digital filter enabled and filtering capability up to 5 tI2CCLK
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter5)
    }
    ///Digital filter enabled and filtering capability up to 6 tI2CCLK
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter6)
    }
    ///Digital filter enabled and filtering capability up to 7 tI2CCLK
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter7)
    }
    ///Digital filter enabled and filtering capability up to 8 tI2CCLK
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter8)
    }
    ///Digital filter enabled and filtering capability up to 9 tI2CCLK
    #[inline(always)]
    pub fn filter9(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter9)
    }
    ///Digital filter enabled and filtering capability up to 10 tI2CCLK
    #[inline(always)]
    pub fn filter10(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter10)
    }
    ///Digital filter enabled and filtering capability up to 11 tI2CCLK
    #[inline(always)]
    pub fn filter11(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter11)
    }
    ///Digital filter enabled and filtering capability up to 12 tI2CCLK
    #[inline(always)]
    pub fn filter12(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter12)
    }
    ///Digital filter enabled and filtering capability up to 13 tI2CCLK
    #[inline(always)]
    pub fn filter13(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter13)
    }
    ///Digital filter enabled and filtering capability up to 14 tI2CCLK
    #[inline(always)]
    pub fn filter14(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter14)
    }
    ///Digital filter enabled and filtering capability up to 15 tI2CCLK
    #[inline(always)]
    pub fn filter15(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter15)
    }
}
/**Analog noise filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANOFF {
    ///0: Analog noise filter enabled
    Enabled = 0,
    ///1: Analog noise filter disabled
    Disabled = 1,
}
impl From<ANOFF> for bool {
    #[inline(always)]
    fn from(variant: ANOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `ANOFF` reader - Analog noise filter
pub type ANOFF_R = crate::BitReader<ANOFF>;
impl ANOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANOFF {
        match self.bits {
            false => ANOFF::Enabled,
            true => ANOFF::Disabled,
        }
    }
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANOFF::Enabled
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANOFF::Disabled
    }
}
///Field `ANOFF` writer - Analog noise filter
pub type ANOFF_W<'a, REG> = crate::BitWriter<'a, REG, ANOFF>;
impl<'a, REG> ANOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANOFF::Enabled)
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANOFF::Disabled)
    }
}
impl R {
    ///Bits 0:3 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Analog noise filter
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTR")
            .field("anoff", &self.anoff())
            .field("dnf", &self.dnf())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W<'_, FLTRrs> {
        DNF_W::new(self, 0)
    }
    ///Bit 4 - Analog noise filter
    #[inline(always)]
    pub fn anoff(&mut self) -> ANOFF_W<'_, FLTRrs> {
        ANOFF_W::new(self, 4)
    }
}
/**FLTR register

You can [`read`](crate::Reg::read) this register and get [`fltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#I2C1:FLTR)*/
pub struct FLTRrs;
impl crate::RegisterSpec for FLTRrs {
    type Ux = u32;
}
///`read()` method returns [`fltr::R`](R) reader structure
impl crate::Readable for FLTRrs {}
///`write(|w| ..)` method takes [`fltr::W`](W) writer structure
impl crate::Writable for FLTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTR to value 0
impl crate::Resettable for FLTRrs {}
