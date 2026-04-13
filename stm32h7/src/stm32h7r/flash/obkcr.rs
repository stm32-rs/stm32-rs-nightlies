///Register `OBKCR` reader
pub type R = crate::R<OBKCRrs>;
///Register `OBKCR` writer
pub type W = crate::W<OBKCRrs>;
///Field `OBKINDEX` reader - Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them.
pub type OBKINDEX_R = crate::FieldReader;
///Field `OBKINDEX` writer - Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them.
pub type OBKINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `NEXTKL` reader - Next key level 10 or 11: reserved
pub type NEXTKL_R = crate::FieldReader;
///Field `NEXTKL` writer - Next key level 10 or 11: reserved
pub type NEXTKL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OBKSIZE` reader - Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key.
pub type OBKSIZE_R = crate::FieldReader;
///Field `OBKSIZE` writer - Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key.
pub type OBKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYPROG` reader - Key program This bit must be set to write option byte keys (keys are read otherwise).
pub type KEYPROG_R = crate::BitReader;
///Field `KEYPROG` writer - Key program This bit must be set to write option byte keys (keys are read otherwise).
pub type KEYPROG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSTART` reader - Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged.
pub type KEYSTART_R = crate::BitReader;
///Field `KEYSTART` writer - Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged.
pub type KEYSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them.
    #[inline(always)]
    pub fn obkindex(&self) -> OBKINDEX_R {
        OBKINDEX_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:9 - Next key level 10 or 11: reserved
    #[inline(always)]
    pub fn nextkl(&self) -> NEXTKL_R {
        NEXTKL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key.
    #[inline(always)]
    pub fn obksize(&self) -> OBKSIZE_R {
        OBKSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 14 - Key program This bit must be set to write option byte keys (keys are read otherwise).
    #[inline(always)]
    pub fn keyprog(&self) -> KEYPROG_R {
        KEYPROG_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged.
    #[inline(always)]
    pub fn keystart(&self) -> KEYSTART_R {
        KEYSTART_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBKCR")
            .field("obkindex", &self.obkindex())
            .field("nextkl", &self.nextkl())
            .field("obksize", &self.obksize())
            .field("keyprog", &self.keyprog())
            .field("keystart", &self.keystart())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Option byte key index This bitfield represents the index of the option byte key in a given hide protection level. Reading keys with index lower that 8, the value is not be available in OBKDRx registers. It is instead sent directly to SAES peripheral. All others keys can be read using OBKDRx registers. Up to 32 keys can be provisioned per hide protection level (0, 1 or 2), provided there is enough space left in the Flash to store them.
    #[inline(always)]
    pub fn obkindex(&mut self) -> OBKINDEX_W<'_, OBKCRrs> {
        OBKINDEX_W::new(self, 0)
    }
    ///Bits 8:9 - Next key level 10 or 11: reserved
    #[inline(always)]
    pub fn nextkl(&mut self) -> NEXTKL_W<'_, OBKCRrs> {
        NEXTKL_W::new(self, 8)
    }
    ///Bits 10:11 - Option byte key size Application must use this bitfield to specify how many bits must be used for the new key. Embedded Flash ignores OBKSIZE during read of option keys because size is stored with the key.
    #[inline(always)]
    pub fn obksize(&mut self) -> OBKSIZE_W<'_, OBKCRrs> {
        OBKSIZE_W::new(self, 10)
    }
    ///Bit 14 - Key program This bit must be set to write option byte keys (keys are read otherwise).
    #[inline(always)]
    pub fn keyprog(&mut self) -> KEYPROG_W<'_, OBKCRrs> {
        KEYPROG_W::new(self, 14)
    }
    ///Bit 15 - Key option start This bit is used to start the option byte key operation defined by the PROG bit. The embedded Flash memory resets START when the corresponding operation has been acknowledged.
    #[inline(always)]
    pub fn keystart(&mut self) -> KEYSTART_W<'_, OBKCRrs> {
        KEYSTART_W::new(self, 15)
    }
}
/**FLASH option byte key control register

You can [`read`](crate::Reg::read) this register and get [`obkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBKCR)*/
pub struct OBKCRrs;
impl crate::RegisterSpec for OBKCRrs {
    type Ux = u32;
}
///`read()` method returns [`obkcr::R`](R) reader structure
impl crate::Readable for OBKCRrs {}
///`write(|w| ..)` method takes [`obkcr::W`](W) writer structure
impl crate::Writable for OBKCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OBKCR to value 0x0c00
impl crate::Resettable for OBKCRrs {
    const RESET_VALUE: u32 = 0x0c00;
}
