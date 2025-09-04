///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `STARTCMD` writer - starts maintenance range command (maintenance operation defined in CACHECMD).
pub type STARTCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHECMD` reader - cache command maintenance operation (clean or clean-and-invalidate an address range)
pub type CACHECMD_R = crate::FieldReader;
///Field `CACHECMD` writer - cache command maintenance operation (clean or clean-and-invalidate an address range)
pub type CACHECMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 1:2 - cache command maintenance operation (clean or clean-and-invalidate an address range)
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("cachecmd", &self.cachecmd())
            .finish()
    }
}
impl W {
    ///Bit 0 - starts maintenance range command (maintenance operation defined in CACHECMD).
    #[inline(always)]
    pub fn startcmd(&mut self) -> STARTCMD_W<CR2rs> {
        STARTCMD_W::new(self, 0)
    }
    ///Bits 1:2 - cache command maintenance operation (clean or clean-and-invalidate an address range)
    #[inline(always)]
    pub fn cachecmd(&mut self) -> CACHECMD_W<CR2rs> {
        CACHECMD_W::new(self, 1)
    }
}
/**CACHEAXI control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CACHEAXI:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
