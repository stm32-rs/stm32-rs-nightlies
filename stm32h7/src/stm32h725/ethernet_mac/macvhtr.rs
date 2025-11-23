///Register `MACVHTR` reader
pub type R = crate::R<MACVHTRrs>;
///Register `MACVHTR` writer
pub type W = crate::W<MACVHTRrs>;
///Field `VLHT` reader - VLAN Hash Table
pub type VLHT_R = crate::FieldReader<u16>;
///Field `VLHT` writer - VLAN Hash Table
pub type VLHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - VLAN Hash Table
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVHTR")
            .field("vlht", &self.vlht())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLAN Hash Table
    #[inline(always)]
    pub fn vlht(&mut self) -> VLHT_W<'_, MACVHTRrs> {
        VLHT_W::new(self, 0)
    }
}
/**VLAN Hash table register

You can [`read`](crate::Reg::read) this register and get [`macvhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#Ethernet_MAC:MACVHTR)*/
pub struct MACVHTRrs;
impl crate::RegisterSpec for MACVHTRrs {
    type Ux = u32;
}
///`read()` method returns [`macvhtr::R`](R) reader structure
impl crate::Readable for MACVHTRrs {}
///`write(|w| ..)` method takes [`macvhtr::W`](W) writer structure
impl crate::Writable for MACVHTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVHTR to value 0
impl crate::Resettable for MACVHTRrs {}
