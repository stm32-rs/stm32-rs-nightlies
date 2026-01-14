///Register `EECR3` reader
pub type R = crate::R<EECR3rs>;
///Register `EECR3` writer
pub type W = crate::W<EECR3rs>;
/**External event 6 filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6F {
    ///0: Filter disabled
    Disabled = 0,
    ///1: f_SAMPLING=f_HRTIM, N=2
    Div1N2 = 1,
    ///2: f_SAMPLING=f_HRTIM, N=4
    Div1N4 = 2,
    ///3: f_SAMPLING=f_HRTIM, N=8
    Div1N8 = 3,
    ///4: f_SAMPLING=f_EEVS/2, N=6
    Div2N6 = 4,
    ///5: f_SAMPLING=f_EEVS/2, N=8
    Div2N8 = 5,
    ///6: f_SAMPLING=f_EEVS/4, N=6
    Div4N6 = 6,
    ///7: f_SAMPLING=f_EEVS/4, N=8
    Div4N8 = 7,
    ///8: f_SAMPLING=f_EEVS/8, N=6
    Div8N6 = 8,
    ///9: f_SAMPLING=f_EEVS/8, N=8
    Div8N8 = 9,
    ///10: f_SAMPLING=f_EEVS/16, N=5
    Div16N5 = 10,
    ///11: f_SAMPLING=f_EEVS/16, N=6
    Div16N6 = 11,
    ///12: f_SAMPLING=f_EEVS/16, N=8
    Div16N8 = 12,
    ///13: f_SAMPLING=f_EEVS/32, N=5
    Div32N5 = 13,
    ///14: f_SAMPLING=f_EEVS/32, N=6
    Div32N6 = 14,
    ///15: f_SAMPLING=f_EEVS/32, N=8
    Div32N8 = 15,
}
impl From<EE6F> for u8 {
    #[inline(always)]
    fn from(variant: EE6F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE6F {
    type Ux = u8;
}
impl crate::IsEnum for EE6F {}
///Field `EE6F` reader - External event 6 filter
pub type EE6F_R = crate::FieldReader<EE6F>;
impl EE6F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE6F {
        match self.bits {
            0 => EE6F::Disabled,
            1 => EE6F::Div1N2,
            2 => EE6F::Div1N4,
            3 => EE6F::Div1N8,
            4 => EE6F::Div2N6,
            5 => EE6F::Div2N8,
            6 => EE6F::Div4N6,
            7 => EE6F::Div4N8,
            8 => EE6F::Div8N6,
            9 => EE6F::Div8N8,
            10 => EE6F::Div16N5,
            11 => EE6F::Div16N6,
            12 => EE6F::Div16N8,
            13 => EE6F::Div32N5,
            14 => EE6F::Div32N6,
            15 => EE6F::Div32N8,
            _ => unreachable!(),
        }
    }
    ///Filter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6F::Disabled
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == EE6F::Div1N2
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == EE6F::Div1N4
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == EE6F::Div1N8
    }
    ///f_SAMPLING=f_EEVS/2, N=6
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == EE6F::Div2N6
    }
    ///f_SAMPLING=f_EEVS/2, N=8
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == EE6F::Div2N8
    }
    ///f_SAMPLING=f_EEVS/4, N=6
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == EE6F::Div4N6
    }
    ///f_SAMPLING=f_EEVS/4, N=8
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == EE6F::Div4N8
    }
    ///f_SAMPLING=f_EEVS/8, N=6
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == EE6F::Div8N6
    }
    ///f_SAMPLING=f_EEVS/8, N=8
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == EE6F::Div8N8
    }
    ///f_SAMPLING=f_EEVS/16, N=5
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == EE6F::Div16N5
    }
    ///f_SAMPLING=f_EEVS/16, N=6
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == EE6F::Div16N6
    }
    ///f_SAMPLING=f_EEVS/16, N=8
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == EE6F::Div16N8
    }
    ///f_SAMPLING=f_EEVS/32, N=5
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == EE6F::Div32N5
    }
    ///f_SAMPLING=f_EEVS/32, N=6
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == EE6F::Div32N6
    }
    ///f_SAMPLING=f_EEVS/32, N=8
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == EE6F::Div32N8
    }
}
///Field `EE6F` writer - External event 6 filter
pub type EE6F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EE6F, crate::Safe>;
impl<'a, REG> EE6F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Disabled)
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div1N2)
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div1N4)
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div1N8)
    }
    ///f_SAMPLING=f_EEVS/2, N=6
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div2N6)
    }
    ///f_SAMPLING=f_EEVS/2, N=8
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div2N8)
    }
    ///f_SAMPLING=f_EEVS/4, N=6
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div4N6)
    }
    ///f_SAMPLING=f_EEVS/4, N=8
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div4N8)
    }
    ///f_SAMPLING=f_EEVS/8, N=6
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div8N6)
    }
    ///f_SAMPLING=f_EEVS/8, N=8
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div8N8)
    }
    ///f_SAMPLING=f_EEVS/16, N=5
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div16N5)
    }
    ///f_SAMPLING=f_EEVS/16, N=6
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div16N6)
    }
    ///f_SAMPLING=f_EEVS/16, N=8
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div16N8)
    }
    ///f_SAMPLING=f_EEVS/32, N=5
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div32N5)
    }
    ///f_SAMPLING=f_EEVS/32, N=6
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div32N6)
    }
    ///f_SAMPLING=f_EEVS/32, N=8
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6F::Div32N8)
    }
}
///Field `EE7F` reader - External event 7 filter
pub use EE6F_R as EE7F_R;
///Field `EE8F` reader - External event 8 filter
pub use EE6F_R as EE8F_R;
///Field `EE9F` reader - External event 9 filter
pub use EE6F_R as EE9F_R;
///Field `EE10F` reader - External event 10 filter
pub use EE6F_R as EE10F_R;
///Field `EE7F` writer - External event 7 filter
pub use EE6F_W as EE7F_W;
///Field `EE8F` writer - External event 8 filter
pub use EE6F_W as EE8F_W;
///Field `EE9F` writer - External event 9 filter
pub use EE6F_W as EE9F_W;
///Field `EE10F` writer - External event 10 filter
pub use EE6F_W as EE10F_W;
/**External event sampling clock division

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEVSD {
    ///0: f_EEVS=f_HRTIM
    Div1 = 0,
    ///1: f_EEVS=f_HRTIM/2
    Div2 = 1,
    ///2: f_EEVS=f_HRTIM/4
    Div4 = 2,
    ///3: f_EEVS=f_HRTIM/8
    Div8 = 3,
}
impl From<EEVSD> for u8 {
    #[inline(always)]
    fn from(variant: EEVSD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EEVSD {
    type Ux = u8;
}
impl crate::IsEnum for EEVSD {}
///Field `EEVSD` reader - External event sampling clock division
pub type EEVSD_R = crate::FieldReader<EEVSD>;
impl EEVSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EEVSD {
        match self.bits {
            0 => EEVSD::Div1,
            1 => EEVSD::Div2,
            2 => EEVSD::Div4,
            3 => EEVSD::Div8,
            _ => unreachable!(),
        }
    }
    ///f_EEVS=f_HRTIM
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == EEVSD::Div1
    }
    ///f_EEVS=f_HRTIM/2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == EEVSD::Div2
    }
    ///f_EEVS=f_HRTIM/4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == EEVSD::Div4
    }
    ///f_EEVS=f_HRTIM/8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == EEVSD::Div8
    }
}
///Field `EEVSD` writer - External event sampling clock division
pub type EEVSD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EEVSD, crate::Safe>;
impl<'a, REG> EEVSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///f_EEVS=f_HRTIM
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(EEVSD::Div1)
    }
    ///f_EEVS=f_HRTIM/2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(EEVSD::Div2)
    }
    ///f_EEVS=f_HRTIM/4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(EEVSD::Div4)
    }
    ///f_EEVS=f_HRTIM/8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(EEVSD::Div8)
    }
}
impl R {
    ///Bits 0:3 - External event 6 filter
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:9 - External event 7 filter
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bits 12:15 - External event 8 filter
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 18:21 - External event 9 filter
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 24:27 - External event 10 filter
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 30:31 - External event sampling clock division
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR3")
            .field("ee6f", &self.ee6f())
            .field("ee7f", &self.ee7f())
            .field("ee8f", &self.ee8f())
            .field("ee9f", &self.ee9f())
            .field("ee10f", &self.ee10f())
            .field("eevsd", &self.eevsd())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - External event 6 filter
    #[inline(always)]
    pub fn ee6f(&mut self) -> EE6F_W<'_, EECR3rs> {
        EE6F_W::new(self, 0)
    }
    ///Bits 6:9 - External event 7 filter
    #[inline(always)]
    pub fn ee7f(&mut self) -> EE7F_W<'_, EECR3rs> {
        EE7F_W::new(self, 6)
    }
    ///Bits 12:15 - External event 8 filter
    #[inline(always)]
    pub fn ee8f(&mut self) -> EE8F_W<'_, EECR3rs> {
        EE8F_W::new(self, 12)
    }
    ///Bits 18:21 - External event 9 filter
    #[inline(always)]
    pub fn ee9f(&mut self) -> EE9F_W<'_, EECR3rs> {
        EE9F_W::new(self, 18)
    }
    ///Bits 24:27 - External event 10 filter
    #[inline(always)]
    pub fn ee10f(&mut self) -> EE10F_W<'_, EECR3rs> {
        EE10F_W::new(self, 24)
    }
    ///Bits 30:31 - External event sampling clock division
    #[inline(always)]
    pub fn eevsd(&mut self) -> EEVSD_W<'_, EECR3rs> {
        EEVSD_W::new(self, 30)
    }
}
/**Timer External Event Control Register 3

You can [`read`](crate::Reg::read) this register and get [`eecr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_Common:EECR3)*/
pub struct EECR3rs;
impl crate::RegisterSpec for EECR3rs {
    type Ux = u32;
}
///`read()` method returns [`eecr3::R`](R) reader structure
impl crate::Readable for EECR3rs {}
///`write(|w| ..)` method takes [`eecr3::W`](W) writer structure
impl crate::Writable for EECR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EECR3 to value 0
impl crate::Resettable for EECR3rs {}
