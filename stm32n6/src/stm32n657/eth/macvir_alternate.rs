///Register `MACVIR_alternate` reader
pub type R = crate::R<MACVIR_ALTERNATErs>;
///Register `MACVIR_alternate` writer
pub type W = crate::W<MACVIR_ALTERNATErs>;
///Field `VLT` reader - VLAN Tag for Transmit Packets
pub type VLT_R = crate::FieldReader<u16>;
///Field `VLT` writer - VLAN Tag for Transmit Packets
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CSVL` reader - C-VLAN or S-VLAN
pub type CSVL_R = crate::BitReader;
///Field `CSVL` writer - C-VLAN or S-VLAN
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - VLAN Tag for Transmit Packets
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 19 - C-VLAN or S-VLAN
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVIR_alternate")
            .field("vlt", &self.vlt())
            .field("csvl", &self.csvl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLAN Tag for Transmit Packets
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W<'_, MACVIR_ALTERNATErs> {
        VLT_W::new(self, 0)
    }
    ///Bit 19 - C-VLAN or S-VLAN
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W<'_, MACVIR_ALTERNATErs> {
        CSVL_W::new(self, 19)
    }
}
/**VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macvir_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACVIR_alternate)*/
pub struct MACVIR_ALTERNATErs;
impl crate::RegisterSpec for MACVIR_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`macvir_alternate::R`](R) reader structure
impl crate::Readable for MACVIR_ALTERNATErs {}
///`write(|w| ..)` method takes [`macvir_alternate::W`](W) writer structure
impl crate::Writable for MACVIR_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVIR_alternate to value 0
impl crate::Resettable for MACVIR_ALTERNATErs {}
