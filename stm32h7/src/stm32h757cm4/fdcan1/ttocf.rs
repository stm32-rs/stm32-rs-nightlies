///Register `TTOCF` reader
pub type R = crate::R<TTOCFrs>;
///Register `TTOCF` writer
pub type W = crate::W<TTOCFrs>;
///Field `OM` reader - Operation Mode
pub type OM_R = crate::FieldReader;
///Field `OM` writer - Operation Mode
pub type OM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GEN` reader - Gap Enable
pub type GEN_R = crate::BitReader;
///Field `GEN` writer - Gap Enable
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TM` reader - Time Master
pub type TM_R = crate::BitReader;
///Field `TM` writer - Time Master
pub type TM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDSDL` reader - LD of Synchronization Deviation Limit
pub type LDSDL_R = crate::FieldReader;
///Field `LDSDL` writer - LD of Synchronization Deviation Limit
pub type LDSDL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IRTO` reader - Initial Reference Trigger Offset
pub type IRTO_R = crate::FieldReader;
///Field `IRTO` writer - Initial Reference Trigger Offset
pub type IRTO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `EECS` reader - Enable External Clock Synchronization
pub type EECS_R = crate::BitReader;
///Field `EECS` writer - Enable External Clock Synchronization
pub type EECS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWL` reader - Application Watchdog Limit
pub type AWL_R = crate::FieldReader;
///Field `AWL` writer - Application Watchdog Limit
pub type AWL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EGTF` reader - Enable Global Time Filtering
pub type EGTF_R = crate::BitReader;
///Field `EGTF` writer - Enable Global Time Filtering
pub type EGTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECC` reader - Enable Clock Calibration
pub type ECC_R = crate::BitReader;
///Field `ECC` writer - Enable Clock Calibration
pub type ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVTP` reader - Event Trigger Polarity
pub type EVTP_R = crate::BitReader;
///Field `EVTP` writer - Event Trigger Polarity
pub type EVTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Operation Mode
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Gap Enable
    #[inline(always)]
    pub fn gen_(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Time Master
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - LD of Synchronization Deviation Limit
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:14 - Initial Reference Trigger Offset
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Enable External Clock Synchronization
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Application Watchdog Limit
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Enable Global Time Filtering
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Enable Clock Calibration
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Event Trigger Polarity
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTOCF")
            .field("om", &self.om())
            .field("gen_", &self.gen_())
            .field("tm", &self.tm())
            .field("ldsdl", &self.ldsdl())
            .field("irto", &self.irto())
            .field("eecs", &self.eecs())
            .field("awl", &self.awl())
            .field("egtf", &self.egtf())
            .field("ecc", &self.ecc())
            .field("evtp", &self.evtp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Operation Mode
    #[inline(always)]
    pub fn om(&mut self) -> OM_W<'_, TTOCFrs> {
        OM_W::new(self, 0)
    }
    ///Bit 3 - Gap Enable
    #[inline(always)]
    pub fn gen_(&mut self) -> GEN_W<'_, TTOCFrs> {
        GEN_W::new(self, 3)
    }
    ///Bit 4 - Time Master
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W<'_, TTOCFrs> {
        TM_W::new(self, 4)
    }
    ///Bits 5:7 - LD of Synchronization Deviation Limit
    #[inline(always)]
    pub fn ldsdl(&mut self) -> LDSDL_W<'_, TTOCFrs> {
        LDSDL_W::new(self, 5)
    }
    ///Bits 8:14 - Initial Reference Trigger Offset
    #[inline(always)]
    pub fn irto(&mut self) -> IRTO_W<'_, TTOCFrs> {
        IRTO_W::new(self, 8)
    }
    ///Bit 15 - Enable External Clock Synchronization
    #[inline(always)]
    pub fn eecs(&mut self) -> EECS_W<'_, TTOCFrs> {
        EECS_W::new(self, 15)
    }
    ///Bits 16:23 - Application Watchdog Limit
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W<'_, TTOCFrs> {
        AWL_W::new(self, 16)
    }
    ///Bit 24 - Enable Global Time Filtering
    #[inline(always)]
    pub fn egtf(&mut self) -> EGTF_W<'_, TTOCFrs> {
        EGTF_W::new(self, 24)
    }
    ///Bit 25 - Enable Clock Calibration
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W<'_, TTOCFrs> {
        ECC_W::new(self, 25)
    }
    ///Bit 26 - Event Trigger Polarity
    #[inline(always)]
    pub fn evtp(&mut self) -> EVTP_W<'_, TTOCFrs> {
        EVTP_W::new(self, 26)
    }
}
/**FDCAN TT Operation Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ttocf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttocf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#FDCAN1:TTOCF)*/
pub struct TTOCFrs;
impl crate::RegisterSpec for TTOCFrs {
    type Ux = u32;
}
///`read()` method returns [`ttocf::R`](R) reader structure
impl crate::Readable for TTOCFrs {}
///`write(|w| ..)` method takes [`ttocf::W`](W) writer structure
impl crate::Writable for TTOCFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTOCF to value 0
impl crate::Resettable for TTOCFrs {}
