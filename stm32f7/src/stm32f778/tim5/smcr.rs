///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Slave mode selection
pub use crate::stm32f778::tim2::smcr::SMS;
///Field `SMS` reader - Slave mode selection
pub use crate::stm32f778::tim2::smcr::SMS_R;
///Field `SMS` writer - Slave mode selection
pub use crate::stm32f778::tim2::smcr::SMS_W;
/**Trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS {
    ///0: Internal Trigger 0 (ITR0)
    Itr0 = 0,
    ///1: Internal Trigger 1 (ITR1)
    Itr1 = 1,
    ///2: Internal Trigger 2 (ITR2)
    Itr2 = 2,
    ///4: TI1 Edge Detector (TI1F_ED)
    Ti1fEd = 4,
    ///5: Filtered Timer Input 1 (TI1FP1)
    Ti1fp1 = 5,
    ///6: Filtered Timer Input 2 (TI2FP2)
    Ti2fp2 = 6,
    ///7: External Trigger input (ETRF)
    Etrf = 7,
}
impl From<TS> for u8 {
    #[inline(always)]
    fn from(variant: TS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS {
    type Ux = u8;
}
impl crate::IsEnum for TS {}
///Field `TS` reader - Trigger selection
pub type TS_R = crate::FieldReader<TS>;
impl TS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TS> {
        match self.bits {
            0 => Some(TS::Itr0),
            1 => Some(TS::Itr1),
            2 => Some(TS::Itr2),
            4 => Some(TS::Ti1fEd),
            5 => Some(TS::Ti1fp1),
            6 => Some(TS::Ti2fp2),
            7 => Some(TS::Etrf),
            _ => None,
        }
    }
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == TS::Itr0
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == TS::Itr1
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == TS::Itr2
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TS::Ti1fEd
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TS::Ti1fp1
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TS::Ti2fp2
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == TS::Etrf
    }
}
///Field `TS` writer - Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TS>;
impl<'a, REG> TS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn itr0(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr0)
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn itr1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr1)
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn itr2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr2)
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti1fEd)
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti1fp1)
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti2fp2)
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn etrf(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Etrf)
    }
}
///External clock enable
pub use crate::stm32f778::tim2::smcr::ECE;
///Field `ECE` reader - External clock enable
pub use crate::stm32f778::tim2::smcr::ECE_R;
///Field `ECE` writer - External clock enable
pub use crate::stm32f778::tim2::smcr::ECE_W;
///External trigger filter
pub use crate::stm32f778::tim2::smcr::ETF;
///Field `ETF` reader - External trigger filter
pub use crate::stm32f778::tim2::smcr::ETF_R;
///Field `ETF` writer - External trigger filter
pub use crate::stm32f778::tim2::smcr::ETF_W;
///External trigger polarity
pub use crate::stm32f778::tim2::smcr::ETP;
///External trigger prescaler
pub use crate::stm32f778::tim2::smcr::ETPS;
///Field `ETPS` reader - External trigger prescaler
pub use crate::stm32f778::tim2::smcr::ETPS_R;
///Field `ETPS` writer - External trigger prescaler
pub use crate::stm32f778::tim2::smcr::ETPS_W;
///Field `ETP` reader - External trigger polarity
pub use crate::stm32f778::tim2::smcr::ETP_R;
///Field `ETP` writer - External trigger polarity
pub use crate::stm32f778::tim2::smcr::ETP_W;
///Master/Slave mode
pub use crate::stm32f778::tim2::smcr::MSM;
///Field `MSM` reader - Master/Slave mode
pub use crate::stm32f778::tim2::smcr::MSM_R;
///Field `MSM` writer - Master/Slave mode
pub use crate::stm32f778::tim2::smcr::MSM_W;
///Field `SMS_3` reader - Slave mode selection - bit 3
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - Slave mode selection - bit 3
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("etp", &self.etp())
            .field("ece", &self.ece())
            .field("etps", &self.etps())
            .field("etf", &self.etf())
            .field("msm", &self.msm())
            .field("ts", &self.ts())
            .field("sms", &self.sms())
            .field("sms_3", &self.sms_3())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<'_, SMCRrs> {
        SMS_3_W::new(self, 16)
    }
}
/**slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#TIM5:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {}
