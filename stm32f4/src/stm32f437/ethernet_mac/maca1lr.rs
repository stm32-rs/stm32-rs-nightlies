///Register `MACA1LR` reader
pub type R = crate::R<MACA1LRrs>;
///Register `MACA1LR` writer
pub type W = crate::W<MACA1LRrs>;
///Field `MACA1LR` reader - MACA1LR
pub type MACA1LR_R = crate::FieldReader<u32>;
///Field `MACA1LR` writer - MACA1LR
pub type MACA1LR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MACA1LR
    #[inline(always)]
    pub fn maca1lr(&self) -> MACA1LR_R {
        MACA1LR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1LR")
            .field("maca1lr", &self.maca1lr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MACA1LR
    #[inline(always)]
    pub fn maca1lr(&mut self) -> MACA1LR_W<'_, MACA1LRrs> {
        MACA1LR_W::new(self, 0)
    }
}
/**Ethernet MAC address1 low register

You can [`read`](crate::Reg::read) this register and get [`maca1lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#Ethernet_MAC:MACA1LR)*/
pub struct MACA1LRrs;
impl crate::RegisterSpec for MACA1LRrs {
    type Ux = u32;
}
///`read()` method returns [`maca1lr::R`](R) reader structure
impl crate::Readable for MACA1LRrs {}
///`write(|w| ..)` method takes [`maca1lr::W`](W) writer structure
impl crate::Writable for MACA1LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA1LR to value 0xffff_ffff
impl crate::Resettable for MACA1LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
