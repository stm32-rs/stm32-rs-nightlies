///Register `MACA1LR` reader
pub type R = crate::R<MACA1LRrs>;
///Register `MACA1LR` writer
pub type W = crate::W<MACA1LRrs>;
///Field `MACA1L` reader - MAC address1 low
pub type MACA1L_R = crate::FieldReader<u32>;
///Field `MACA1L` writer - MAC address1 low
pub type MACA1L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC address1 low
    #[inline(always)]
    pub fn maca1l(&self) -> MACA1L_R {
        MACA1L_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1LR")
            .field("maca1l", &self.maca1l())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC address1 low
    #[inline(always)]
    pub fn maca1l(&mut self) -> MACA1L_W<'_, MACA1LRrs> {
        MACA1L_W::new(self, 0)
    }
}
/**Ethernet MAC address1 low register

You can [`read`](crate::Reg::read) this register and get [`maca1lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#Ethernet_MAC:MACA1LR)*/
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
