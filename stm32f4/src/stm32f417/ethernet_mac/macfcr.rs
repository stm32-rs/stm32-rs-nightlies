///Register `MACFCR` reader
pub type R = crate::R<MACFCRrs>;
///Register `MACFCR` writer
pub type W = crate::W<MACFCRrs>;
///Field `FCB` reader - FCB
pub type FCB_R = crate::BitReader;
///Field `FCB` writer - FCB
pub type FCB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFCE` reader - TFCE
pub type TFCE_R = crate::BitReader;
///Field `TFCE` writer - TFCE
pub type TFCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFCE` reader - RFCE
pub type RFCE_R = crate::BitReader;
///Field `RFCE` writer - RFCE
pub type RFCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPFD` reader - UPFD
pub type UPFD_R = crate::BitReader;
///Field `UPFD` writer - UPFD
pub type UPFD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLT` reader - PLT
pub type PLT_R = crate::FieldReader;
///Field `PLT` writer - PLT
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ZQPD` reader - ZQPD
pub type ZQPD_R = crate::BitReader;
///Field `ZQPD` writer - ZQPD
pub type ZQPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PT` reader - PT
pub type PT_R = crate::FieldReader<u16>;
///Field `PT` writer - PT
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - FCB
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TFCE
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RFCE
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - UPFD
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - PLT
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - ZQPD
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:31 - PT
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFCR")
            .field("fcb", &self.fcb())
            .field("tfce", &self.tfce())
            .field("rfce", &self.rfce())
            .field("upfd", &self.upfd())
            .field("plt", &self.plt())
            .field("zqpd", &self.zqpd())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    ///Bit 0 - FCB
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W<'_, MACFCRrs> {
        FCB_W::new(self, 0)
    }
    ///Bit 1 - TFCE
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W<'_, MACFCRrs> {
        TFCE_W::new(self, 1)
    }
    ///Bit 2 - RFCE
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W<'_, MACFCRrs> {
        RFCE_W::new(self, 2)
    }
    ///Bit 3 - UPFD
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W<'_, MACFCRrs> {
        UPFD_W::new(self, 3)
    }
    ///Bits 4:5 - PLT
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, MACFCRrs> {
        PLT_W::new(self, 4)
    }
    ///Bit 7 - ZQPD
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W<'_, MACFCRrs> {
        ZQPD_W::new(self, 7)
    }
    ///Bits 16:31 - PT
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, MACFCRrs> {
        PT_W::new(self, 16)
    }
}
/**Ethernet MAC flow control register

You can [`read`](crate::Reg::read) this register and get [`macfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_MAC:MACFCR)*/
pub struct MACFCRrs;
impl crate::RegisterSpec for MACFCRrs {
    type Ux = u32;
}
///`read()` method returns [`macfcr::R`](R) reader structure
impl crate::Readable for MACFCRrs {}
///`write(|w| ..)` method takes [`macfcr::W`](W) writer structure
impl crate::Writable for MACFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACFCR to value 0
impl crate::Resettable for MACFCRrs {}
