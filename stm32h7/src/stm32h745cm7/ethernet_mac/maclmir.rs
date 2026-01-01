///Register `MACLMIR` reader
pub type R = crate::R<MACLMIRrs>;
///Register `MACLMIR` writer
pub type W = crate::W<MACLMIRrs>;
///Field `LSI` reader - Log Sync Interval
pub type LSI_R = crate::FieldReader;
///Field `LSI` writer - Log Sync Interval
pub type LSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DRSYNCR` reader - Delay_Req to SYNC Ratio
pub type DRSYNCR_R = crate::FieldReader;
///Field `DRSYNCR` writer - Delay_Req to SYNC Ratio
pub type DRSYNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LMPDRI` reader - Log Min Pdelay_Req Interval
pub type LMPDRI_R = crate::FieldReader;
///Field `LMPDRI` writer - Log Min Pdelay_Req Interval
pub type LMPDRI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Log Sync Interval
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - Delay_Req to SYNC Ratio
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 24:31 - Log Min Pdelay_Req Interval
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACLMIR")
            .field("lsi", &self.lsi())
            .field("drsyncr", &self.drsyncr())
            .field("lmpdri", &self.lmpdri())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Log Sync Interval
    #[inline(always)]
    pub fn lsi(&mut self) -> LSI_W<'_, MACLMIRrs> {
        LSI_W::new(self, 0)
    }
    ///Bits 8:10 - Delay_Req to SYNC Ratio
    #[inline(always)]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<'_, MACLMIRrs> {
        DRSYNCR_W::new(self, 8)
    }
    ///Bits 24:31 - Log Min Pdelay_Req Interval
    #[inline(always)]
    pub fn lmpdri(&mut self) -> LMPDRI_W<'_, MACLMIRrs> {
        LMPDRI_W::new(self, 24)
    }
}
/**Log message interval register

You can [`read`](crate::Reg::read) this register and get [`maclmir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclmir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#Ethernet_MAC:MACLMIR)*/
pub struct MACLMIRrs;
impl crate::RegisterSpec for MACLMIRrs {
    type Ux = u32;
}
///`read()` method returns [`maclmir::R`](R) reader structure
impl crate::Readable for MACLMIRrs {}
///`write(|w| ..)` method takes [`maclmir::W`](W) writer structure
impl crate::Writable for MACLMIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACLMIR to value 0
impl crate::Resettable for MACLMIRrs {}
