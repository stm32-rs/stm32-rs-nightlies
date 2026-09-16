///Register `SMPR` reader
pub type R = crate::R<SMPRrs>;
///Register `SMPR` writer
pub type W = crate::W<SMPRrs>;
///Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMP1_R = crate::FieldReader;
///Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMP2_R = crate::FieldReader;
///Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMPSEL0` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL0_R = crate::BitReader;
///Field `SMPSEL0` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL1` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL1_R = crate::BitReader;
///Field `SMPSEL1` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL2` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL2_R = crate::BitReader;
///Field `SMPSEL2` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL3` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL3_R = crate::BitReader;
///Field `SMPSEL3` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL4` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL4_R = crate::BitReader;
///Field `SMPSEL4` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL5` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL5_R = crate::BitReader;
///Field `SMPSEL5` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL6` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL6_R = crate::BitReader;
///Field `SMPSEL6` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL7` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL7_R = crate::BitReader;
///Field `SMPSEL7` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL8` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL8_R = crate::BitReader;
///Field `SMPSEL8` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL9` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL9_R = crate::BitReader;
///Field `SMPSEL9` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL10` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL10_R = crate::BitReader;
///Field `SMPSEL10` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL11` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL11_R = crate::BitReader;
///Field `SMPSEL11` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL12` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL12_R = crate::BitReader;
///Field `SMPSEL12` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL13` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL13_R = crate::BitReader;
///Field `SMPSEL13` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL14` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL14_R = crate::BitReader;
///Field `SMPSEL14` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL15` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL15_R = crate::BitReader;
///Field `SMPSEL15` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL16` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL16_R = crate::BitReader;
///Field `SMPSEL16` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL17` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL17_R = crate::BitReader;
///Field `SMPSEL17` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL18` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL18_R = crate::BitReader;
///Field `SMPSEL18` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEL19` reader - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL19_R = crate::BitReader;
///Field `SMPSEL19` writer - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
pub type SMPSEL19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel19(&self) -> SMPSEL19_R {
        SMPSEL19_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR")
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smpsel0", &self.smpsel0())
            .field("smpsel1", &self.smpsel1())
            .field("smpsel2", &self.smpsel2())
            .field("smpsel3", &self.smpsel3())
            .field("smpsel4", &self.smpsel4())
            .field("smpsel5", &self.smpsel5())
            .field("smpsel6", &self.smpsel6())
            .field("smpsel7", &self.smpsel7())
            .field("smpsel8", &self.smpsel8())
            .field("smpsel9", &self.smpsel9())
            .field("smpsel10", &self.smpsel10())
            .field("smpsel11", &self.smpsel11())
            .field("smpsel12", &self.smpsel12())
            .field("smpsel13", &self.smpsel13())
            .field("smpsel14", &self.smpsel14())
            .field("smpsel15", &self.smpsel15())
            .field("smpsel16", &self.smpsel16())
            .field("smpsel17", &self.smpsel17())
            .field("smpsel18", &self.smpsel18())
            .field("smpsel19", &self.smpsel19())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<'_, SMPRrs> {
        SMP1_W::new(self, 0)
    }
    ///Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<'_, SMPRrs> {
        SMP2_W::new(self, 4)
    }
    ///Bit 8 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<'_, SMPRrs> {
        SMPSEL0_W::new(self, 8)
    }
    ///Bit 9 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<'_, SMPRrs> {
        SMPSEL1_W::new(self, 9)
    }
    ///Bit 10 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<'_, SMPRrs> {
        SMPSEL2_W::new(self, 10)
    }
    ///Bit 11 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<'_, SMPRrs> {
        SMPSEL3_W::new(self, 11)
    }
    ///Bit 12 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<'_, SMPRrs> {
        SMPSEL4_W::new(self, 12)
    }
    ///Bit 13 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<'_, SMPRrs> {
        SMPSEL5_W::new(self, 13)
    }
    ///Bit 14 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<'_, SMPRrs> {
        SMPSEL6_W::new(self, 14)
    }
    ///Bit 15 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<'_, SMPRrs> {
        SMPSEL7_W::new(self, 15)
    }
    ///Bit 16 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<'_, SMPRrs> {
        SMPSEL8_W::new(self, 16)
    }
    ///Bit 17 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<'_, SMPRrs> {
        SMPSEL9_W::new(self, 17)
    }
    ///Bit 18 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<'_, SMPRrs> {
        SMPSEL10_W::new(self, 18)
    }
    ///Bit 19 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<'_, SMPRrs> {
        SMPSEL11_W::new(self, 19)
    }
    ///Bit 20 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<'_, SMPRrs> {
        SMPSEL12_W::new(self, 20)
    }
    ///Bit 21 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<'_, SMPRrs> {
        SMPSEL13_W::new(self, 21)
    }
    ///Bit 22 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<'_, SMPRrs> {
        SMPSEL14_W::new(self, 22)
    }
    ///Bit 23 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<'_, SMPRrs> {
        SMPSEL15_W::new(self, 23)
    }
    ///Bit 24 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<'_, SMPRrs> {
        SMPSEL16_W::new(self, 24)
    }
    ///Bit 25 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<'_, SMPRrs> {
        SMPSEL17_W::new(self, 25)
    }
    ///Bit 26 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<'_, SMPRrs> {
        SMPSEL18_W::new(self, 26)
    }
    ///Bit 27 - Channel-x sampling time selection (x1=119 to 0) These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART1=10 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel19(&mut self) -> SMPSEL19_W<'_, SMPRrs> {
        SMPSEL19_W::new(self, 27)
    }
}
/**ADC sampling time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#ADC:SMPR)*/
pub struct SMPRrs;
impl crate::RegisterSpec for SMPRrs {
    type Ux = u32;
}
///`read()` method returns [`smpr::R`](R) reader structure
impl crate::Readable for SMPRrs {}
///`write(|w| ..)` method takes [`smpr::W`](W) writer structure
impl crate::Writable for SMPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR to value 0
impl crate::Resettable for SMPRrs {}
