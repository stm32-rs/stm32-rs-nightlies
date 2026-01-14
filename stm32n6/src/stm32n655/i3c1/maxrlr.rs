///Register `MAXRLR` reader
pub type R = crate::R<MAXRLRrs>;
///Register `MAXRLR` writer
pub type W = crate::W<MAXRLRrs>;
///Field `MRL` reader - Maximum data read length (when I3C acts as target)
pub type MRL_R = crate::FieldReader<u16>;
///Field `MRL` writer - Maximum data read length (when I3C acts as target)
pub type MRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `IBIP` reader - IBI payload data maximum size, in bytes (when I3C acts as target)
pub type IBIP_R = crate::FieldReader;
///Field `IBIP` writer - IBI payload data maximum size, in bytes (when I3C acts as target)
pub type IBIP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:15 - Maximum data read length (when I3C acts as target)
    #[inline(always)]
    pub fn mrl(&self) -> MRL_R {
        MRL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - IBI payload data maximum size, in bytes (when I3C acts as target)
    #[inline(always)]
    pub fn ibip(&self) -> IBIP_R {
        IBIP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAXRLR")
            .field("mrl", &self.mrl())
            .field("ibip", &self.ibip())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Maximum data read length (when I3C acts as target)
    #[inline(always)]
    pub fn mrl(&mut self) -> MRL_W<'_, MAXRLRrs> {
        MRL_W::new(self, 0)
    }
    ///Bits 16:18 - IBI payload data maximum size, in bytes (when I3C acts as target)
    #[inline(always)]
    pub fn ibip(&mut self) -> IBIP_W<'_, MAXRLRrs> {
        IBIP_W::new(self, 16)
    }
}
/**I3C maximum read length register

You can [`read`](crate::Reg::read) this register and get [`maxrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#I3C1:MAXRLR)*/
pub struct MAXRLRrs;
impl crate::RegisterSpec for MAXRLRrs {
    type Ux = u32;
}
///`read()` method returns [`maxrlr::R`](R) reader structure
impl crate::Readable for MAXRLRrs {}
///`write(|w| ..)` method takes [`maxrlr::W`](W) writer structure
impl crate::Writable for MAXRLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAXRLR to value 0
impl crate::Resettable for MAXRLRrs {}
