///Register `B0CR` reader
pub type R = crate::R<B0CRrs>;
///Register `B0CR` writer
pub type W = crate::W<B0CRrs>;
///Field `PBO` reader - Physical buffer offset
pub type PBO_R = crate::FieldReader<u32>;
///Field `PBO` writer - Physical buffer offset
pub type PBO_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PBBA` reader - Physical buffer base address
pub type PBBA_R = crate::FieldReader<u16>;
///Field `PBBA` writer - Physical buffer base address
pub type PBBA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 4:22 - Physical buffer offset
    #[inline(always)]
    pub fn pbo(&self) -> PBO_R {
        PBO_R::new((self.bits >> 4) & 0x0007_ffff)
    }
    ///Bits 23:31 - Physical buffer base address
    #[inline(always)]
    pub fn pbba(&self) -> PBBA_R {
        PBBA_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B0CR")
            .field("pbo", &self.pbo())
            .field("pbba", &self.pbba())
            .finish()
    }
}
impl W {
    ///Bits 4:22 - Physical buffer offset
    #[inline(always)]
    pub fn pbo(&mut self) -> PBO_W<B0CRrs> {
        PBO_W::new(self, 4)
    }
    ///Bits 23:31 - Physical buffer base address
    #[inline(always)]
    pub fn pbba(&mut self) -> PBBA_W<B0CRrs> {
        PBBA_W::new(self, 23)
    }
}
/**GFXMMU buffer 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`b0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXMMU:B0CR)*/
pub struct B0CRrs;
impl crate::RegisterSpec for B0CRrs {
    type Ux = u32;
}
///`read()` method returns [`b0cr::R`](R) reader structure
impl crate::Readable for B0CRrs {}
///`write(|w| ..)` method takes [`b0cr::W`](W) writer structure
impl crate::Writable for B0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets B0CR to value 0
impl crate::Resettable for B0CRrs {}
