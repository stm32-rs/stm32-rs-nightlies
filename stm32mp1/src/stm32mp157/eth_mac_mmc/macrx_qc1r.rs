///Register `MACRxQC1R` reader
pub type R = crate::R<MACRX_QC1Rrs>;
///Register `MACRxQC1R` writer
pub type W = crate::W<MACRX_QC1Rrs>;
///Field `AVCPQ` reader - AVCPQ
pub type AVCPQ_R = crate::FieldReader;
///Field `AVCPQ` writer - AVCPQ
pub type AVCPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AVPTPQ` reader - AVPTPQ
pub type AVPTPQ_R = crate::FieldReader;
///Field `AVPTPQ` writer - AVPTPQ
pub type AVPTPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UPQ` reader - UPQ
pub type UPQ_R = crate::FieldReader;
///Field `UPQ` writer - UPQ
pub type UPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCBCQ` reader - MCBCQ
pub type MCBCQ_R = crate::FieldReader;
///Field `MCBCQ` writer - MCBCQ
pub type MCBCQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCBCQEN` reader - MCBCQEN
pub type MCBCQEN_R = crate::BitReader;
///Field `MCBCQEN` writer - MCBCQEN
pub type MCBCQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TACPQE` reader - TACPQE
pub type TACPQE_R = crate::BitReader;
///Field `TACPQE` writer - TACPQE
pub type TACPQE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - AVCPQ
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - AVPTPQ
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - UPQ
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - MCBCQ
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - MCBCQEN
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TACPQE
    #[inline(always)]
    pub fn tacpqe(&self) -> TACPQE_R {
        TACPQE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRxQC1R")
            .field("avcpq", &self.avcpq())
            .field("avptpq", &self.avptpq())
            .field("upq", &self.upq())
            .field("mcbcq", &self.mcbcq())
            .field("mcbcqen", &self.mcbcqen())
            .field("tacpqe", &self.tacpqe())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AVCPQ
    #[inline(always)]
    pub fn avcpq(&mut self) -> AVCPQ_W<'_, MACRX_QC1Rrs> {
        AVCPQ_W::new(self, 0)
    }
    ///Bits 4:6 - AVPTPQ
    #[inline(always)]
    pub fn avptpq(&mut self) -> AVPTPQ_W<'_, MACRX_QC1Rrs> {
        AVPTPQ_W::new(self, 4)
    }
    ///Bits 12:14 - UPQ
    #[inline(always)]
    pub fn upq(&mut self) -> UPQ_W<'_, MACRX_QC1Rrs> {
        UPQ_W::new(self, 12)
    }
    ///Bits 16:18 - MCBCQ
    #[inline(always)]
    pub fn mcbcq(&mut self) -> MCBCQ_W<'_, MACRX_QC1Rrs> {
        MCBCQ_W::new(self, 16)
    }
    ///Bit 20 - MCBCQEN
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W<'_, MACRX_QC1Rrs> {
        MCBCQEN_W::new(self, 20)
    }
    ///Bit 21 - TACPQE
    #[inline(always)]
    pub fn tacpqe(&mut self) -> TACPQE_W<'_, MACRX_QC1Rrs> {
        TACPQE_W::new(self, 21)
    }
}
/**The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.

You can [`read`](crate::Reg::read) this register and get [`macrx_qc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_qc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACRxQC1R)*/
pub struct MACRX_QC1Rrs;
impl crate::RegisterSpec for MACRX_QC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macrx_qc1r::R`](R) reader structure
impl crate::Readable for MACRX_QC1Rrs {}
///`write(|w| ..)` method takes [`macrx_qc1r::W`](W) writer structure
impl crate::Writable for MACRX_QC1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRxQC1R to value 0
impl crate::Resettable for MACRX_QC1Rrs {}
