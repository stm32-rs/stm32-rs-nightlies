///Register `MACTSEACR` reader
pub type R = crate::R<MACTSEACRrs>;
///Register `MACTSEACR` writer
pub type W = crate::W<MACTSEACRrs>;
///Field `OSTEAC` reader - One-Step Timestamp Egress Asymmetry Correction
pub type OSTEAC_R = crate::FieldReader<u32>;
///Field `OSTEAC` writer - One-Step Timestamp Egress Asymmetry Correction
pub type OSTEAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSEACR")
            .field("osteac", &self.osteac())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction
    #[inline(always)]
    pub fn osteac(&mut self) -> OSTEAC_W<'_, MACTSEACRrs> {
        OSTEAC_W::new(self, 0)
    }
}
/**Timestamp Egress asymmetric correction register

You can [`read`](crate::Reg::read) this register and get [`mactseacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactseacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#Ethernet_MAC:MACTSEACR)*/
pub struct MACTSEACRrs;
impl crate::RegisterSpec for MACTSEACRrs {
    type Ux = u32;
}
///`read()` method returns [`mactseacr::R`](R) reader structure
impl crate::Readable for MACTSEACRrs {}
///`write(|w| ..)` method takes [`mactseacr::W`](W) writer structure
impl crate::Writable for MACTSEACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSEACR to value 0
impl crate::Resettable for MACTSEACRrs {}
