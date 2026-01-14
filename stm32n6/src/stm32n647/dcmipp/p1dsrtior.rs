///Register `P1DSRTIOR` reader
pub type R = crate::R<P1DSRTIORrs>;
///Register `P1DSRTIOR` writer
pub type W = crate::W<P1DSRTIORrs>;
///Field `HRATIO` reader - Horizontal ratio, from 8192 (1x) to 65535 (8x)
pub type HRATIO_R = crate::FieldReader<u16>;
///Field `HRATIO` writer - Horizontal ratio, from 8192 (1x) to 65535 (8x)
pub type HRATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `VRATIO` reader - Vertical ratio, from 8192 (1x) to 65535 (8x)
pub type VRATIO_R = crate::FieldReader<u16>;
///Field `VRATIO` writer - Vertical ratio, from 8192 (1x) to 65535 (8x)
pub type VRATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Horizontal ratio, from 8192 (1x) to 65535 (8x)
    #[inline(always)]
    pub fn hratio(&self) -> HRATIO_R {
        HRATIO_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Vertical ratio, from 8192 (1x) to 65535 (8x)
    #[inline(always)]
    pub fn vratio(&self) -> VRATIO_R {
        VRATIO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1DSRTIOR")
            .field("hratio", &self.hratio())
            .field("vratio", &self.vratio())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Horizontal ratio, from 8192 (1x) to 65535 (8x)
    #[inline(always)]
    pub fn hratio(&mut self) -> HRATIO_W<'_, P1DSRTIORrs> {
        HRATIO_W::new(self, 0)
    }
    ///Bits 16:31 - Vertical ratio, from 8192 (1x) to 65535 (8x)
    #[inline(always)]
    pub fn vratio(&mut self) -> VRATIO_W<'_, P1DSRTIORrs> {
        VRATIO_W::new(self, 16)
    }
}
/**DCMIPP Pipex downsize ratio register

You can [`read`](crate::Reg::read) this register and get [`p1dsrtior::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dsrtior::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1DSRTIOR)*/
pub struct P1DSRTIORrs;
impl crate::RegisterSpec for P1DSRTIORrs {
    type Ux = u32;
}
///`read()` method returns [`p1dsrtior::R`](R) reader structure
impl crate::Readable for P1DSRTIORrs {}
///`write(|w| ..)` method takes [`p1dsrtior::W`](W) writer structure
impl crate::Writable for P1DSRTIORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1DSRTIOR to value 0
impl crate::Resettable for P1DSRTIORrs {}
