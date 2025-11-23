///Register `REG11_BNESTR` reader
pub type R = crate::R<REG11_BNESTRrs>;
///Register `REG11_BNESTR` writer
pub type W = crate::W<REG11_BNESTRrs>;
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
        f.debug_struct("REG11_BNESTR")
            .field("dcen", &self.dcen())
            .finish()
    }
}
impl W {
    ///Bit 2 - delegated configuration enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<'_, REG11_BNESTRrs> {
        DCEN_W::new(self, 2)
    }
}
/**RISAF region 11 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg11_bnestr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_bnestr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG11_BNESTR)*/
pub struct REG11_BNESTRrs;
impl crate::RegisterSpec for REG11_BNESTRrs {
    type Ux = u32;
}
///`read()` method returns [`reg11_bnestr::R`](R) reader structure
impl crate::Readable for REG11_BNESTRrs {}
///`write(|w| ..)` method takes [`reg11_bnestr::W`](W) writer structure
impl crate::Writable for REG11_BNESTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG11_BNESTR to value 0
impl crate::Resettable for REG11_BNESTRrs {}
