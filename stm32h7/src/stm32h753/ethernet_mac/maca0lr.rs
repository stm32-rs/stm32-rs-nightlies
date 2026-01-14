///Register `MACA0LR` reader
pub type R = crate::R<MACA0LRrs>;
///Register `MACA0LR` writer
pub type W = crate::W<MACA0LRrs>;
///Field `ADDRLO` reader - MAC Address 0 \[31:0\]
pub type ADDRLO_R = crate::FieldReader<u32>;
///Field `ADDRLO` writer - MAC Address 0 \[31:0\]
pub type ADDRLO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC Address 0 \[31:0\]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0LR")
            .field("addrlo", &self.addrlo())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC Address 0 \[31:0\]
    #[inline(always)]
    pub fn addrlo(&mut self) -> ADDRLO_W<'_, MACA0LRrs> {
        ADDRLO_W::new(self, 0)
    }
}
/**Address 0 low register

You can [`read`](crate::Reg::read) this register and get [`maca0lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#Ethernet_MAC:MACA0LR)*/
pub struct MACA0LRrs;
impl crate::RegisterSpec for MACA0LRrs {
    type Ux = u32;
}
///`read()` method returns [`maca0lr::R`](R) reader structure
impl crate::Readable for MACA0LRrs {}
///`write(|w| ..)` method takes [`maca0lr::W`](W) writer structure
impl crate::Writable for MACA0LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA0LR to value 0xffff_ffff
impl crate::Resettable for MACA0LRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
