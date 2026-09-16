///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `ETTXE` reader - EoTp Transmission Enable
pub type ETTXE_R = crate::BitReader;
///Field `ETTXE` writer - EoTp Transmission Enable
pub type ETTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETRXE` reader - EoTp Reception Enable
pub type ETRXE_R = crate::BitReader;
///Field `ETRXE` writer - EoTp Reception Enable
pub type ETRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTAE` reader - Bus Turn Around Enable
pub type BTAE_R = crate::BitReader;
///Field `BTAE` writer - Bus Turn Around Enable
pub type BTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCRXE` reader - ECC Reception Enable
pub type ECCRXE_R = crate::BitReader;
///Field `ECCRXE` writer - ECC Reception Enable
pub type ECCRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRXE` reader - CRC Reception Enable
pub type CRCRXE_R = crate::BitReader;
///Field `CRCRXE` writer - CRC Reception Enable
pub type CRCRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EoTp Transmission Enable
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EoTp Reception Enable
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bus Turn Around Enable
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC Reception Enable
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC Reception Enable
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("ettxe", &self.ettxe())
            .field("etrxe", &self.etrxe())
            .field("btae", &self.btae())
            .field("eccrxe", &self.eccrxe())
            .field("crcrxe", &self.crcrxe())
            .finish()
    }
}
impl W {
    ///Bit 0 - EoTp Transmission Enable
    #[inline(always)]
    pub fn ettxe(&mut self) -> ETTXE_W<'_, PCRrs> {
        ETTXE_W::new(self, 0)
    }
    ///Bit 1 - EoTp Reception Enable
    #[inline(always)]
    pub fn etrxe(&mut self) -> ETRXE_W<'_, PCRrs> {
        ETRXE_W::new(self, 1)
    }
    ///Bit 2 - Bus Turn Around Enable
    #[inline(always)]
    pub fn btae(&mut self) -> BTAE_W<'_, PCRrs> {
        BTAE_W::new(self, 2)
    }
    ///Bit 3 - ECC Reception Enable
    #[inline(always)]
    pub fn eccrxe(&mut self) -> ECCRXE_W<'_, PCRrs> {
        ECCRXE_W::new(self, 3)
    }
    ///Bit 4 - CRC Reception Enable
    #[inline(always)]
    pub fn crcrxe(&mut self) -> CRCRXE_W<'_, PCRrs> {
        CRCRXE_W::new(self, 4)
    }
}
/**DSI Host Protocol Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#DSI:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR to value 0
impl crate::Resettable for PCRrs {}
