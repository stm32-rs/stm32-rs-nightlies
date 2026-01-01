///Register `REG8_ANESTR` reader
pub type R = crate::R<REG8_ANESTRrs>;
///Register `REG8_ANESTR` writer
pub type W = crate::W<REG8_ANESTRrs>;
///Field `DCEN` reader - delegated configuration enable
pub type DCEN_R = crate::BitReader;
///Field `DCEN` writer - delegated configuration enable
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - delegated configuration enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG8_ANESTR")
            .field("dcen", &self.dcen())
            .finish()
    }
}
impl W {
    ///Bit 2 - delegated configuration enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, REG8_ANESTRrs> {
        DCEN_W::new(self, 2)
    }
}
/**RISAF region 8 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg8_anestr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_anestr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RISAF:REG8_ANESTR)*/
pub struct REG8_ANESTRrs;
impl crate::RegisterSpec for REG8_ANESTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg8_anestr::R`](R) reader structure
impl crate::Readable for REG8_ANESTRrs {}
///`write(|w| ..)` method takes [`reg8_anestr::W`](W) writer structure
impl crate::Writable for REG8_ANESTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG8_ANESTR to value 0
impl crate::Resettable for REG8_ANESTRrs {}
