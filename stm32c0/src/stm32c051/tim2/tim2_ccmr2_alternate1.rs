///Register `TIM2_CCMR2_ALTERNATE1` reader
pub type R = crate::R<TIM2_CCMR2_ALTERNATE1rs>;
///Register `TIM2_CCMR2_ALTERNATE1` writer
pub type W = crate::W<TIM2_CCMR2_ALTERNATE1rs>;
/**Capture/Compare 3 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S {
    ///0: CC3 channel is configured as output
    B0x0 = 0,
    ///1: CC3 channel is configured as input, IC3 is mapped on TI3
    B0x1 = 1,
    ///2: CC3 channel is configured as input, IC3 is mapped on TI4
    B0x2 = 2,
    ///3: CC3 channel is configured as input, IC3 is mapped on TRC.
    B0x3 = 3,
}
impl From<CC3S> for u8 {
    #[inline(always)]
    fn from(variant: CC3S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC3S {
    type Ux = u8;
}
impl crate::IsEnum for CC3S {}
///Field `CC3S` reader - Capture/Compare 3 selection
pub type CC3S_R = crate::FieldReader<CC3S>;
impl CC3S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC3S {
        match self.bits {
            0 => CC3S::B0x0,
            1 => CC3S::B0x1,
            2 => CC3S::B0x2,
            3 => CC3S::B0x3,
            _ => unreachable!(),
        }
    }
    ///CC3 channel is configured as output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC3S::B0x0
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC3S::B0x1
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC3S::B0x2
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC3S::B0x3
    }
}
///Field `CC3S` writer - Capture/Compare 3 selection
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S, crate::Safe>;
impl<'a, REG> CC3S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC3 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x0)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x1)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x2)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x3)
    }
}
///Field `OC3FE` reader - Output compare 3 fast enable
pub type OC3FE_R = crate::BitReader;
///Field `OC3FE` writer - Output compare 3 fast enable
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3PE` reader - Output compare 3 preload enable
pub type OC3PE_R = crate::BitReader;
///Field `OC3PE` writer - Output compare 3 preload enable
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M` reader - OC3M\[2:0\]: Output compare 3 mode
pub type OC3M_R = crate::FieldReader;
///Field `OC3M` writer - OC3M\[2:0\]: Output compare 3 mode
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC3CE` reader - Output compare 3 clear enable
pub type OC3CE_R = crate::BitReader;
///Field `OC3CE` writer - Output compare 3 clear enable
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Capture/Compare 4 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S {
    ///0: CC4 channel is configured as output
    B0x0 = 0,
    ///1: CC4 channel is configured as input, IC4 is mapped on TI4
    B0x1 = 1,
    ///2: CC4 channel is configured as input, IC4 is mapped on TI3
    B0x2 = 2,
    ///3: CC4 channel is configured as input, IC4 is mapped on TRC.
    B0x3 = 3,
}
impl From<CC4S> for u8 {
    #[inline(always)]
    fn from(variant: CC4S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC4S {
    type Ux = u8;
}
impl crate::IsEnum for CC4S {}
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = crate::FieldReader<CC4S>;
impl CC4S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC4S {
        match self.bits {
            0 => CC4S::B0x0,
            1 => CC4S::B0x1,
            2 => CC4S::B0x2,
            3 => CC4S::B0x3,
            _ => unreachable!(),
        }
    }
    ///CC4 channel is configured as output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC4S::B0x0
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC4S::B0x1
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC4S::B0x2
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC4S::B0x3
    }
}
///Field `CC4S` writer - Capture/Compare 4 selection
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC4S, crate::Safe>;
impl<'a, REG> CC4S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC4 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x0)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x1)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x2)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x3)
    }
}
///Field `OC4FE` reader - Output compare 4 fast enable
pub type OC4FE_R = crate::BitReader;
///Field `OC4FE` writer - Output compare 4 fast enable
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4PE` reader - Output compare 4 preload enable
pub type OC4PE_R = crate::BitReader;
///Field `OC4PE` writer - Output compare 4 preload enable
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M` reader - OC4M\[2:0\]: Output compare 4 mode
pub type OC4M_R = crate::FieldReader;
///Field `OC4M` writer - OC4M\[2:0\]: Output compare 4 mode
pub type OC4M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC4CE` reader - Output compare 4 clear enable
pub type OC4CE_R = crate::BitReader;
///Field `OC4CE` writer - Output compare 4 clear enable
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC3M_1` reader - OC3M\[3\]
pub type OC3M_1_R = crate::BitReader;
///Field `OC3M_1` writer - OC3M\[3\]
pub type OC3M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC4M_1` reader - OC4M\[3\]
pub type OC4M_1_R = crate::BitReader;
///Field `OC4M_1` writer - OC4M\[3\]
pub type OC4M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC3M\[2:0\]: Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC4M\[2:0\]: Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OC3M\[3\]
    #[inline(always)]
    pub fn oc3m_1(&self) -> OC3M_1_R {
        OC3M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC4M\[3\]
    #[inline(always)]
    pub fn oc4m_1(&self) -> OC4M_1_R {
        OC4M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CCMR2_ALTERNATE1")
            .field("cc3s", &self.cc3s())
            .field("oc3fe", &self.oc3fe())
            .field("oc3pe", &self.oc3pe())
            .field("oc3m", &self.oc3m())
            .field("oc3ce", &self.oc3ce())
            .field("cc4s", &self.cc4s())
            .field("oc4fe", &self.oc4fe())
            .field("oc4pe", &self.oc4pe())
            .field("oc4m", &self.oc4m())
            .field("oc4ce", &self.oc4ce())
            .field("oc3m_1", &self.oc3m_1())
            .field("oc4m_1", &self.oc4m_1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        CC3S_W::new(self, 0)
    }
    ///Bit 2 - Output compare 3 fast enable
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC3FE_W::new(self, 2)
    }
    ///Bit 3 - Output compare 3 preload enable
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC3PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC3M\[2:0\]: Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC3M_W::new(self, 4)
    }
    ///Bit 7 - Output compare 3 clear enable
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC3CE_W::new(self, 7)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        CC4S_W::new(self, 8)
    }
    ///Bit 10 - Output compare 4 fast enable
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC4FE_W::new(self, 10)
    }
    ///Bit 11 - Output compare 4 preload enable
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC4PE_W::new(self, 11)
    }
    ///Bits 12:14 - OC4M\[2:0\]: Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC4M_W::new(self, 12)
    }
    ///Bit 15 - Output compare 4 clear enable
    #[inline(always)]
    pub fn oc4ce(&mut self) -> OC4CE_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC4CE_W::new(self, 15)
    }
    ///Bit 16 - OC3M\[3\]
    #[inline(always)]
    pub fn oc3m_1(&mut self) -> OC3M_1_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC3M_1_W::new(self, 16)
    }
    ///Bit 24 - OC4M\[3\]
    #[inline(always)]
    pub fn oc4m_1(&mut self) -> OC4M_1_W<'_, TIM2_CCMR2_ALTERNATE1rs> {
        OC4M_1_W::new(self, 24)
    }
}
/**TIM2 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr2_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr2_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM2:TIM2_CCMR2_ALTERNATE1)*/
pub struct TIM2_CCMR2_ALTERNATE1rs;
impl crate::RegisterSpec for TIM2_CCMR2_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_ccmr2_alternate1::R`](R) reader structure
impl crate::Readable for TIM2_CCMR2_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`tim2_ccmr2_alternate1::W`](W) writer structure
impl crate::Writable for TIM2_CCMR2_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CCMR2_ALTERNATE1 to value 0
impl crate::Resettable for TIM2_CCMR2_ALTERNATE1rs {}
