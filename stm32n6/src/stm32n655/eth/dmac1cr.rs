///Register `DMAC1CR` reader
pub type R = crate::R<DMAC1CRrs>;
///Register `DMAC1CR` writer
pub type W = crate::W<DMAC1CRrs>;
///Field `MSS` reader - Maximum Segment Size
pub type MSS_R = crate::FieldReader<u16>;
///Field `MSS` writer - Maximum Segment Size
pub type MSS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `PBLX8` reader - 8xPBL mode
pub type PBLX8_R = crate::BitReader;
///Field `PBLX8` writer - 8xPBL mode
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSL` reader - Descriptor Skip Length
pub type DSL_R = crate::FieldReader;
///Field `DSL` writer - Descriptor Skip Length
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:13 - Maximum Segment Size
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - 8xPBL mode
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:20 - Descriptor Skip Length
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1CR")
            .field("mss", &self.mss())
            .field("pblx8", &self.pblx8())
            .field("dsl", &self.dsl())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Maximum Segment Size
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W<'_, DMAC1CRrs> {
        MSS_W::new(self, 0)
    }
    ///Bit 16 - 8xPBL mode
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<'_, DMAC1CRrs> {
        PBLX8_W::new(self, 16)
    }
    ///Bits 18:20 - Descriptor Skip Length
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<'_, DMAC1CRrs> {
        DSL_W::new(self, 18)
    }
}
/**Channel 1 control register

You can [`read`](crate::Reg::read) this register and get [`dmac1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CR)*/
pub struct DMAC1CRrs;
impl crate::RegisterSpec for DMAC1CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1cr::R`](R) reader structure
impl crate::Readable for DMAC1CRrs {}
///`write(|w| ..)` method takes [`dmac1cr::W`](W) writer structure
impl crate::Writable for DMAC1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1CR to value 0
impl crate::Resettable for DMAC1CRrs {}
