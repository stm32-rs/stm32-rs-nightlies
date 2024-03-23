#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    OnCompare = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    OnUpdate = 1,
}
impl From<CCDS> for bool {
    #[inline(always)]
    fn from(variant: CCDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<CCDS>;
impl CCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCDS {
        match self.bits {
            false => CCDS::OnCompare,
            true => CCDS::OnUpdate,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS::OnCompare
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS::OnUpdate
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG, CCDS>;
impl<'a, REG> CCDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::OnCompare)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::OnUpdate)
    }
}
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    #[doc = "0: The UG bit from the TIMx_EGR register is used as trigger output"]
    Reset = 0,
    #[doc = "1: The counter enable signal, CNT_EN, is used as trigger output"]
    Enable = 1,
    #[doc = "2: The update event is selected as trigger output"]
    Update = 2,
    #[doc = "3: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    ComparePulse = 3,
    #[doc = "4: OC1REF signal is used as trigger output"]
    CompareOc1 = 4,
    #[doc = "5: OC2REF signal is used as trigger output"]
    CompareOc2 = 5,
    #[doc = "6: OC3REF signal is used as trigger output"]
    CompareOc3 = 6,
    #[doc = "7: OC4REF signal is used as trigger output"]
    CompareOc4 = 7,
}
impl From<MMS> for u8 {
    #[inline(always)]
    fn from(variant: MMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS {
    type Ux = u8;
}
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMS {
        match self.bits {
            0 => MMS::Reset,
            1 => MMS::Enable,
            2 => MMS::Update,
            3 => MMS::ComparePulse,
            4 => MMS::CompareOc1,
            5 => MMS::CompareOc2,
            6 => MMS::CompareOc3,
            7 => MMS::CompareOc4,
            _ => unreachable!(),
        }
    }
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS::Reset
    }
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS::Enable
    }
    #[doc = "The update event is selected as trigger output"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS::Update
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS::ComparePulse
    }
    #[doc = "OC1REF signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMS::CompareOc1
    }
    #[doc = "OC2REF signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMS::CompareOc2
    }
    #[doc = "OC3REF signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMS::CompareOc3
    }
    #[doc = "OC4REF signal is used as trigger output"]
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMS::CompareOc4
    }
}
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MMS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MMS>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Reset)
    }
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Enable)
    }
    #[doc = "The update event is selected as trigger output"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Update)
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::ComparePulse)
    }
    #[doc = "OC1REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc1)
    }
    #[doc = "OC2REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc2)
    }
    #[doc = "OC3REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc3)
    }
    #[doc = "OC4REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc4)
    }
}
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    Normal = 0,
    #[doc = "1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    Xor = 1,
}
impl From<TI1S> for bool {
    #[inline(always)]
    fn from(variant: TI1S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type TI1S_R = crate::BitReader<TI1S>;
impl TI1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1S {
        match self.bits {
            false => TI1S::Normal,
            true => TI1S::Xor,
        }
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI1S::Normal
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI1S::Xor
    }
}
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG, TI1S>;
impl<'a, REG> TI1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::Normal)
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::Xor)
    }
}
impl R {
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
