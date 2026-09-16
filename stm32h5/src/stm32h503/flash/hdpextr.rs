///Register `HDPEXTR` reader
pub type R = crate::R<HDPEXTRrs>;
///Register `HDPEXTR` writer
pub type W = crate::W<HDPEXTRrs>;
///Field `HDP1_EXT` reader - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector.
pub type HDP1_EXT_R = crate::FieldReader;
///Field `HDP1_EXT` writer - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector.
pub type HDP1_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HDP2_EXT` reader - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector.
pub type HDP2_EXT_R = crate::FieldReader;
///Field `HDP2_EXT` writer - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector.
pub type HDP2_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector.
    #[inline(always)]
    pub fn hdp1_ext(&self) -> HDP1_EXT_R {
        HDP1_EXT_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector.
    #[inline(always)]
    pub fn hdp2_ext(&self) -> HDP2_EXT_R {
        HDP2_EXT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDPEXTR")
            .field("hdp1_ext", &self.hdp1_ext())
            .field("hdp2_ext", &self.hdp2_ext())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector.
    #[inline(always)]
    pub fn hdp1_ext(&mut self) -> HDP1_EXT_W<'_, HDPEXTRrs> {
        HDP1_EXT_W::new(self, 0)
    }
    ///Bits 16:18 - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector.
    #[inline(always)]
    pub fn hdp2_ext(&mut self) -> HDP2_EXT_W<'_, HDPEXTRrs> {
        HDP2_EXT_W::new(self, 16)
    }
}
/**FLASH HDP extension register

You can [`read`](crate::Reg::read) this register and get [`hdpextr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpextr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDPEXTR)*/
pub struct HDPEXTRrs;
impl crate::RegisterSpec for HDPEXTRrs {
    type Ux = u32;
}
///`read()` method returns [`hdpextr::R`](R) reader structure
impl crate::Readable for HDPEXTRrs {}
///`write(|w| ..)` method takes [`hdpextr::W`](W) writer structure
impl crate::Writable for HDPEXTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HDPEXTR to value 0
impl crate::Resettable for HDPEXTRrs {}
