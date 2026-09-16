///Register `DX2GCR` reader
pub type R = crate::R<DX2GCRrs>;
///Register `DX2GCR` writer
pub type W = crate::W<DX2GCRrs>;
///Field `DXEN` reader - DXEN
pub type DXEN_R = crate::BitReader;
///Field `DXEN` writer - DXEN
pub type DXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSODT` reader - DQSODT
pub type DQSODT_R = crate::BitReader;
///Field `DQSODT` writer - DQSODT
pub type DQSODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQODT` reader - DQODT
pub type DQODT_R = crate::BitReader;
///Field `DQODT` writer - DQODT
pub type DQODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXIOM` reader - DXIOM
pub type DXIOM_R = crate::BitReader;
///Field `DXIOM` writer - DXIOM
pub type DXIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXPDD` reader - DXPDD
pub type DXPDD_R = crate::BitReader;
///Field `DXPDD` writer - DXPDD
pub type DXPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXPDR` reader - DXPDR
pub type DXPDR_R = crate::BitReader;
///Field `DXPDR` writer - DXPDR
pub type DXPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSRPD` reader - DQSRPD
pub type DQSRPD_R = crate::BitReader;
///Field `DQSRPD` writer - DQSRPD
pub type DQSRPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEN` reader - DSEN
pub type DSEN_R = crate::FieldReader;
///Field `DSEN` writer - DSEN
pub type DSEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DQSRTT` reader - DQSRTT
pub type DQSRTT_R = crate::BitReader;
///Field `DQSRTT` writer - DQSRTT
pub type DQSRTT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQRTT` reader - DQRTT
pub type DQRTT_R = crate::BitReader;
///Field `DQRTT` writer - DQRTT
pub type DQRTT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTTOH` reader - RTTOH
pub type RTTOH_R = crate::FieldReader;
///Field `RTTOH` writer - RTTOH
pub type RTTOH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTTOAL` reader - RTTOAL
pub type RTTOAL_R = crate::BitReader;
///Field `RTTOAL` writer - RTTOAL
pub type RTTOAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R0RVSL` reader - R0RVSL
pub type R0RVSL_R = crate::FieldReader;
///Field `R0RVSL` writer - R0RVSL
pub type R0RVSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - DXEN
    #[inline(always)]
    pub fn dxen(&self) -> DXEN_R {
        DXEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DQSODT
    #[inline(always)]
    pub fn dqsodt(&self) -> DQSODT_R {
        DQSODT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DQODT
    #[inline(always)]
    pub fn dqodt(&self) -> DQODT_R {
        DQODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DXIOM
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DXPDD
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DXPDR
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DQSRPD
    #[inline(always)]
    pub fn dqsrpd(&self) -> DQSRPD_R {
        DQSRPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - DSEN
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - DQSRTT
    #[inline(always)]
    pub fn dqsrtt(&self) -> DQSRTT_R {
        DQSRTT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DQRTT
    #[inline(always)]
    pub fn dqrtt(&self) -> DQRTT_R {
        DQRTT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - RTTOH
    #[inline(always)]
    pub fn rttoh(&self) -> RTTOH_R {
        RTTOH_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - RTTOAL
    #[inline(always)]
    pub fn rttoal(&self) -> RTTOAL_R {
        RTTOAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:16 - R0RVSL
    #[inline(always)]
    pub fn r0rvsl(&self) -> R0RVSL_R {
        R0RVSL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DX2GCR")
            .field("dxen", &self.dxen())
            .field("dqsodt", &self.dqsodt())
            .field("dqodt", &self.dqodt())
            .field("dxiom", &self.dxiom())
            .field("dxpdd", &self.dxpdd())
            .field("dxpdr", &self.dxpdr())
            .field("dqsrpd", &self.dqsrpd())
            .field("dsen", &self.dsen())
            .field("dqsrtt", &self.dqsrtt())
            .field("dqrtt", &self.dqrtt())
            .field("rttoh", &self.rttoh())
            .field("rttoal", &self.rttoal())
            .field("r0rvsl", &self.r0rvsl())
            .finish()
    }
}
impl W {
    ///Bit 0 - DXEN
    #[inline(always)]
    pub fn dxen(&mut self) -> DXEN_W<'_, DX2GCRrs> {
        DXEN_W::new(self, 0)
    }
    ///Bit 1 - DQSODT
    #[inline(always)]
    pub fn dqsodt(&mut self) -> DQSODT_W<'_, DX2GCRrs> {
        DQSODT_W::new(self, 1)
    }
    ///Bit 2 - DQODT
    #[inline(always)]
    pub fn dqodt(&mut self) -> DQODT_W<'_, DX2GCRrs> {
        DQODT_W::new(self, 2)
    }
    ///Bit 3 - DXIOM
    #[inline(always)]
    pub fn dxiom(&mut self) -> DXIOM_W<'_, DX2GCRrs> {
        DXIOM_W::new(self, 3)
    }
    ///Bit 4 - DXPDD
    #[inline(always)]
    pub fn dxpdd(&mut self) -> DXPDD_W<'_, DX2GCRrs> {
        DXPDD_W::new(self, 4)
    }
    ///Bit 5 - DXPDR
    #[inline(always)]
    pub fn dxpdr(&mut self) -> DXPDR_W<'_, DX2GCRrs> {
        DXPDR_W::new(self, 5)
    }
    ///Bit 6 - DQSRPD
    #[inline(always)]
    pub fn dqsrpd(&mut self) -> DQSRPD_W<'_, DX2GCRrs> {
        DQSRPD_W::new(self, 6)
    }
    ///Bits 7:8 - DSEN
    #[inline(always)]
    pub fn dsen(&mut self) -> DSEN_W<'_, DX2GCRrs> {
        DSEN_W::new(self, 7)
    }
    ///Bit 9 - DQSRTT
    #[inline(always)]
    pub fn dqsrtt(&mut self) -> DQSRTT_W<'_, DX2GCRrs> {
        DQSRTT_W::new(self, 9)
    }
    ///Bit 10 - DQRTT
    #[inline(always)]
    pub fn dqrtt(&mut self) -> DQRTT_W<'_, DX2GCRrs> {
        DQRTT_W::new(self, 10)
    }
    ///Bits 11:12 - RTTOH
    #[inline(always)]
    pub fn rttoh(&mut self) -> RTTOH_W<'_, DX2GCRrs> {
        RTTOH_W::new(self, 11)
    }
    ///Bit 13 - RTTOAL
    #[inline(always)]
    pub fn rttoal(&mut self) -> RTTOAL_W<'_, DX2GCRrs> {
        RTTOAL_W::new(self, 13)
    }
    ///Bits 14:16 - R0RVSL
    #[inline(always)]
    pub fn r0rvsl(&mut self) -> R0RVSL_W<'_, DX2GCRrs> {
        R0RVSL_W::new(self, 14)
    }
}
/**DDRPHYC byte lane 2 GC register

You can [`read`](crate::Reg::read) this register and get [`dx2gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx2gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX2GCR)*/
pub struct DX2GCRrs;
impl crate::RegisterSpec for DX2GCRrs {
    type Ux = u32;
}
///`read()` method returns [`dx2gcr::R`](R) reader structure
impl crate::Readable for DX2GCRrs {}
///`write(|w| ..)` method takes [`dx2gcr::W`](W) writer structure
impl crate::Writable for DX2GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DX2GCR to value 0xee81
impl crate::Resettable for DX2GCRrs {
    const RESET_VALUE: u32 = 0xee81;
}
