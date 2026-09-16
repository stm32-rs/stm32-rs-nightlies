///Register `CEC_CFGR` reader
pub type R = crate::R<CEC_CFGRrs>;
///Register `CEC_CFGR` writer
pub type W = crate::W<CEC_CFGRrs>;
///Field `SFT` reader - Signal Free Time
pub type SFT_R = crate::FieldReader;
///Field `SFT` writer - Signal Free Time
pub type SFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXTOL` reader - Rx-Tolerance
pub type RXTOL_R = crate::BitReader;
///Field `RXTOL` writer - Rx-Tolerance
pub type RXTOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRESTP` reader - Rx-Stop on Bit Rising Error
pub type BRESTP_R = crate::BitReader;
///Field `BRESTP` writer - Rx-Stop on Bit Rising Error
pub type BRESTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREGEN` reader - Generate Error-Bit on Bit Rising Error
pub type BREGEN_R = crate::BitReader;
///Field `BREGEN` writer - Generate Error-Bit on Bit Rising Error
pub type BREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error
pub type LBPEGEN_R = crate::BitReader;
///Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error
pub type LBPEGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRDNOGEN` reader - Avoid Error-Bit Generation in Broadcast
pub type BRDNOGEN_R = crate::BitReader;
///Field `BRDNOGEN` writer - Avoid Error-Bit Generation in Broadcast
pub type BRDNOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFTOP` reader - SFT Option Bit
pub type SFTOP_R = crate::BitReader;
///Field `SFTOP` writer - SFT Option Bit
pub type SFTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OAR` reader - Own addresses configuration
pub type OAR_R = crate::FieldReader<u16>;
///Field `OAR` writer - Own addresses configuration
pub type OAR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `LSTN` reader - Listen mode
pub type LSTN_R = crate::BitReader;
///Field `LSTN` writer - Listen mode
pub type LSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Signal Free Time
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Rx-Tolerance
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx-Stop on Bit Rising Error
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Generate Error-Bit on Bit Rising Error
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Generate Error-Bit on Long Bit Period Error
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Avoid Error-Bit Generation in Broadcast
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SFT Option Bit
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:30 - Own addresses configuration
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - Listen mode
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEC_CFGR")
            .field("lstn", &self.lstn())
            .field("oar", &self.oar())
            .field("sftop", &self.sftop())
            .field("brdnogen", &self.brdnogen())
            .field("lbpegen", &self.lbpegen())
            .field("bregen", &self.bregen())
            .field("brestp", &self.brestp())
            .field("rxtol", &self.rxtol())
            .field("sft", &self.sft())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Signal Free Time
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W<'_, CEC_CFGRrs> {
        SFT_W::new(self, 0)
    }
    ///Bit 3 - Rx-Tolerance
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W<'_, CEC_CFGRrs> {
        RXTOL_W::new(self, 3)
    }
    ///Bit 4 - Rx-Stop on Bit Rising Error
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W<'_, CEC_CFGRrs> {
        BRESTP_W::new(self, 4)
    }
    ///Bit 5 - Generate Error-Bit on Bit Rising Error
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W<'_, CEC_CFGRrs> {
        BREGEN_W::new(self, 5)
    }
    ///Bit 6 - Generate Error-Bit on Long Bit Period Error
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<'_, CEC_CFGRrs> {
        LBPEGEN_W::new(self, 6)
    }
    ///Bit 7 - Avoid Error-Bit Generation in Broadcast
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<'_, CEC_CFGRrs> {
        BRDNOGEN_W::new(self, 7)
    }
    ///Bit 8 - SFT Option Bit
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W<'_, CEC_CFGRrs> {
        SFTOP_W::new(self, 8)
    }
    ///Bits 16:30 - Own addresses configuration
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W<'_, CEC_CFGRrs> {
        OAR_W::new(self, 16)
    }
    ///Bit 31 - Listen mode
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W<'_, CEC_CFGRrs> {
        LSTN_W::new(self, 31)
    }
}
/**CEC configuration register

You can [`read`](crate::Reg::read) this register and get [`cec_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_CFGR)*/
pub struct CEC_CFGRrs;
impl crate::RegisterSpec for CEC_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cec_cfgr::R`](R) reader structure
impl crate::Readable for CEC_CFGRrs {}
///`write(|w| ..)` method takes [`cec_cfgr::W`](W) writer structure
impl crate::Writable for CEC_CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEC_CFGR to value 0
impl crate::Resettable for CEC_CFGRrs {}
